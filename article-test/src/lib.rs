#[doc = include_str!("../../articles/serde-pyobject.md")]
pub mod serde_pyobject {}

#[doc = include_str!("../../articles/tokio-task-cancel.md")]
pub mod tokio_task_cancel {}

#[doc = include_str!("../../articles/async-timeout.md")]
pub mod async_timeout {
    #[derive(Default)]
    pub struct PendingOnce {
        polled: bool, // boolのデフォルトはfalse
    }

    impl std::future::Future for PendingOnce {
        type Output = ();

        fn poll(
            mut self: std::pin::Pin<&mut Self>,
            cx: &mut std::task::Context<'_>,
        ) -> std::task::Poll<()> {
            if self.polled {
                // 既に１回ポーリングされているので終了
                std::task::Poll::Ready(())
            } else {
                // 初回だけPendingを返す
                self.polled = true; // ２回目以降はReadyを返す

                // Pendingを返した直後からもうこのFutureは準備できているのでwakeを呼んでおく
                cx.waker().wake_by_ref();

                std::task::Poll::Pending
            }
        }
    }
}
