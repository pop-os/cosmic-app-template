    /// Alarm edit view
    fn alarm_edit_view(&self, edit: &AlarmEdit) -> Element<Message> {
        let cosmic_theme::Spacing { space_m, space_l, .. } = theme::active().cosmic().spacing;
        
        let hour_str = edit.hour.to_string();
        let minute_str = edit.minute.to_string();

        widget::column()
            .push(widget::text::title2(fl!("add-alarm")))
            .push(
                widget::row()
                    .push(widget::text::body(fl!("hour")))
                    .push(
                        widget::text_input("", &hour_str)
                            .on_input(|s| Message::AlarmEditHour(s.parse().unwrap_or(0)))
                    )
                    .push(widget::text::body(fl!("minute")))
                    .push(
                        widget::text_input("", &minute_str)
                            .on_input(|s| Message::AlarmEditMinute(s.parse().unwrap_or(0)))
                    )
                    .spacing(space_m)
                    .align_y(Vertical::Center)
            )
            .push(
                widget::text_input(fl!("enter-label"), &edit.label)
                    .on_input(Message::AlarmEditLabel)
            )
            .push(
                widget::row()
                    .push(widget::button::standard(fl!("save-alarm")).on_press(Message::SaveAlarm))
                    .push(widget::button::standard(fl!("cancel")).on_press(Message::CancelAlarmEdit))
                    .spacing(space_m)
            )
            .spacing(space_m)
            .align_x(Alignment::Center)
            .apply(widget::container)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center)
            .padding(space_l)
            .into()
    }
