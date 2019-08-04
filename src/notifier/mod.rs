extern crate notify_rust;
use notify_rust::Notification;
use crate::PACKAGE_NAME;

pub fn notify(notification_body: &str) {
    Notification::new()
        .summary(PACKAGE_NAME)
        .body(notification_body)
        .icon("dialog-information")
        .show().unwrap();
}