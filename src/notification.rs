pub mod notification {
    use notify_rust::Notification;

    pub fn send_notification(summary: &str, body: &str) {
        let notification_summary = summary.to_string();
        let notification_body = body.to_string();
        std::thread::spawn(move || {
            notify(notification_summary.as_str(), notification_body.as_str())
        })
        .join()
        .unwrap();
    }

    pub fn notify(summary: &str, body: &str) {
        Notification::new()
            .summary(summary)
            .body(body)
            .show()
            .unwrap();
    }
}
