---
title: "serdeから型の情報を取り出す"
emoji: "♻"
type: "tech"
topics: ["rust", "serde"]
published: true
---

serdeの持っている情報から実行時の型タグを生成するcrateをリリースしました。

https://github.com/termoshtt/serde-typeinfo/releases/tag/v0.1.0

```rust
use serde_typeinfo::{TypeTag, Primitive};
use serde::Serialize;

#[derive(Serialize)]
struct A {
    a: u8,
    b: u8,
}

assert_eq!(
    type_of_value(&A { a: 2, b: 3 }),
    TypeTag::Struct {
        name: "A",
        fields: vec![
            ("a", Primitive::U8.into()),
            ("b", Primitive::U8.into()),
        ]
    }
);
```

このように構造体`A`が`"A"`という名前の構造体であって、`u8`型の`"a"`と`"b"`という2つのfieldを持つ事を表す実行時のタグ`TypeTag`を生成します。これは擬似的な実行時型情報として使うことが出来ます。ただしあくまでserdeの意味で(serde data modelで)同じになる構造体については同じタグを返すので、Rustの意味で異なる構造体に対して同じタグを返す可能性があります。

先行研究
---------
元々rust-jpのZulipで
https://github.com/eduidl/tysh
が紹介されていた事がきっかけで思いついたので作りました。これは型のIDを型の名前やfield名から生成するためのcrateです。

同じような試みとしてFFI/interop ABIの文脈で
https://github.com/h33p/ctti
というcrateが公開されています。例えばC++とRustのようにC ABIよりもリッチな型を持つ言語間でFFIを行うときに型の情報をどうやってエンコードしておくかという問題に取り組むためのcrateです。作者のブログが印象的です。
https://blaz.is/blog/post/we-dont-need-a-stable-abi/
Rustのinterop ABIの提案は以下です。以前WebAssembly Component model / Interface typeを調べたのもこれの関係でした。
https://github.com/rust-lang/rust/pull/105586
https://zenn.dev/termoshtt/articles/webassembly-component-model

serde data model
-----------------
Rustの有名なシリアライズフレームワークとしてserdeがありますが、これは複数の構造体をJSONのような複数のデータフォーマットに対応させるため[serde data model](https://serde.rs/data-model.html)を介して仮想的に2段階のマッピングを行います：

- ユーザーの定義した構造体をserde data model対応させる(`Serialize` trait)
- serde data modelをデータフォーマットに対応させる(`Serializer` trait)

このserde data modelと呼ばれる中間データモデルは実行時に生成されるのではなく、TraitのAPIとして定義されます。つまりユーザー定義の構造体に対して`serde::Serialize`の実装自体がこの構造体からserde data modelへの対応を定義していて、例えばJSONを生成する`JSONSerializer`のような型への`serde::Serializer`の実装によってserde data modelからデータフォーマットへの対応が定義されています。このようにすることで実行時のコストをかけずに任意のデータフォーマットへの変換が定義できます。

serde data modelにおいて区別出来ないRustの構造体はデータフォーマット(`Serializer`)側では区別できなくなるので、serde data modelはRust側のデータモデルをなるべく表現できる形に次の16種類用意されています：

- 14 primitive types
  - `bool`, `i8`, `i16`, `i32`, `i64`, `i128`, `u8`, `u16`, `u32`, `u64`, `u128`, `f32`, `f64`, `char`
- string `&str`
- byte array `[u8]`
- option
- unit `()`
- unit_struct
  - 名前がついているけど値の無いデータ、例えば`struct Unit`や`PhantomData<T>`
- unit_variant
  - `enum E { A, B }`における`E::A`や`E::B`
- newtype_struct
  - 別の型に名前をつけたもの、例えば`struct Millimeters(u8)`
- newtype_variant
  - `enum E { N(u8) }`における`E::N`
- seq
  - 長さが変化する非一様な型のリスト。値によって対応するデータが異なるRustの型、例えば`enum Value { Char(char), U8(u8) }`のような場合に`vec![Value::Char('c'), Value::U8(1)]`はtuple_structの`Value::Char`と`Value::U8`のリストになる。
- tuple
  - 長さが変化しない非一様な型のリスト。
- tuple_struct
  - 名前付きタプル、例えば`struct Rgb(u8, u8, u8)`
- tuple_variant
  - `enum E { T(u8, u8) }`における`E::T`
- map
- struct
- struct_variant
  - `enum E { S { r: u8, g: u8, b: u8 } }`における`E::S`

ユーザーは自分の定義した構造体からこれらへの対応を自分で実装することもでき、また`#[derive(serde::Serialize)]`によって自動的に導出する事もできます。

一方データフォーマットはこれらのAPIによって伝えられてきた構造体の名前やフィールド名のようなメタデータと値を受け取って実際のフォーマットに書き出していきます。`Serializer`のAPIは結構ややこしいですが、いくつかかい摘んで見ていきましょう。今回はデータフォーマットとしては特殊になってしまいますが、`serde-typeinfo`の実装ケースを解説していきます。`TypeTagSerializer`という空の型を作ってそれに`Serializer`を実装していきます：

```rust
impl ser::Serializer for TypeTagSerializer {
    type Ok = TypeTag;
    type Error = Error;
    ...
}
```

まずは例によってシリアライズした結果の型`Ok`とエラー型`Error`を定義します。`Ok`を`()`にしておいて`TypeTagSerializer`相当の構造体が内部で保持してるバッファーに書き込んでいく方法もあります。

データとしてプリミティブ型、例えば`u32`が入力されたときは`Serializer::serialize_u32`が呼ばれます。今回作る`serde-typeinfo`では受け取ったデータモデルからデータを捨ててその値の所属しているデータを表すタグ(`TypeTag`)のみを生成する`Serializer`を作ります。

```rust
impl ser::Serializer for TypeTagSerializer {
    fn serialize_u32(self, _value: u32) -> Result<Self::Ok> {
        Ok(TypeTag::Primitive(Primitive::U32))
    }
}
```

APIによって対応が定義されていると言っていたのはこのことで、data modelにおける`u32`のデータが来たときにデータフォーマット側でどう処理するかを定めるのがこの関数です。

`serde-typeinfo`にとって重要なのは`unit_struct`のようにdata modelに名前の情報が含まれている場合で、この場合は引数に`&'static str`として名前が渡されます：

```rust
impl ser::Serializer for TypeTagSerializer {
    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok> {
        Ok(TypeTag::UnitStruct { name })
    }
}
```

これは例えば`struct Unit`に対しては`name`に`"Unit"`が入れて渡されます。これは`#[derive(Serialize)]`の実装が`Serializer::serialize_unit_struct`を呼び出すときに手続きマクロで収集した構造体の名前を実装中に埋め込んでいるので可能となっています。

最後に`seq`や`tuple`あるいは`struct`のように別の基本的な型を組み合わせて生成される型の場合を見ましょう。この場合は一旦サブの構造体に処理を移します。その中で一番単純な`seq`の場合を見てみましょう：

```rust
impl ser::Serializer for TypeTagSerializer {
    type SerializeSeq = TypeTagSeq;
    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        Ok(TypeTagSeq {
            seq: Seq::default(),
        })
    }
}

#[derive(Debug)]
pub struct TypeTagSeq {
    seq: Seq,
}

impl ser::SerializeSeq for TypeTagSeq {
    type Ok = TypeTag;
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        let tag = TypeTag::from_value(value);
        self.seq.push(tag);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(TypeTag::Seq(self.seq))
    }
}
```

`Seq`の定義は割愛します。`TypeTag::from_value`は`TypeTagSerializer`を使ってシリアライズを行う関数なので、この部分で実質的に再帰していて、再帰降下的にプリミティブ型になるまで分解して定義します。

利点と欠点
-----------
この方法の利点は`#[derive(serde::Serialize)]`の定義をそのまま使える事です。もし独自の実行時タグの為にTraitを実装する必要があるとすると、既存のライブラリにそのTraitの実装を追加する必要がありますが、それが他人の管理しているものであれば難しいでしょう。

一方で最初に述べた通り、この方法で定義したタグはserde data modelにおいて一致する構造体を区別することは出来ず、この点が受け入れられない用途には使えません。
