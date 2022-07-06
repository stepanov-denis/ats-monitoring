pub mod gateway {
    /// SMS Gateway string connection
    pub fn sms_gateway_string_connection() -> Option<String> {
        Some(String::from("https://api-mapper.clicksend.com/http/v2/send.php?method=http&username=development-service@yandex.ru&key=1E82A334-89D8-985C-526B-712DB70A713D&to=+79139402913&message="))
    }

    /// The text of the SMS-message about the generator operation error
    pub fn sms_generator_work_err() -> Option<String> {
        let mut string_connection =
            String::from(sms_gateway_string_connection().unwrap_or_default());
        string_connection
            .push_str("Авария!+Генератор+неисправен!+Срочно+произведите+сервисные+работы!");
        Some(string_connection)
    }

    /// The text of the Telegram-message about the generator operation error
    pub fn _tg_msg_generator_work_err() -> Option<String> {
        Some(String::from("Авария! Генератор неисправен! Срочно произведите сервисные работы!"))
    }

    /// The text of the SMS-message about the resumption of the generator operation
    pub fn sms_generator_work_restored() -> Option<String> {
        let mut string_connection =
            String::from(sms_gateway_string_connection().unwrap_or_default());
        string_connection.push_str("Работоспособность+генератора+в+режиме+трансляции+питания+от+электросети+восстановлена.+Генератор+исправен.+Генератор+работает.");
        Some(string_connection)
    }

    /// The text of the Telegram-message about the resumption of the generator operation
    pub fn _tg_msg_generator_work_restored() -> Option<String> {
        Some(String::from("Работоспособность генератора в режиме трансляции питания от электросети восстановлена.\nГенератор+исправен.\nГенератор работает."))
    }

    /// The text of the SMS-message about the successful start of the generator
    pub fn sms_start_generator_ok() -> Option<String> {
        let mut string_connection =
            String::from(sms_gateway_string_connection().unwrap_or_default());
        string_connection.push_str("Сбой+питания+от+электросети.+Успешный+старт+генератора.");
        Some(string_connection)
    }

    /// The text of the Telegram-message about the successful start of the generator
    pub fn _tg_msg_start_generator_ok() -> Option<String> {
        Some(String::from("Сбой питания от электросети.\nУспешный старт генератора."))
    }

    /// Text of SMS-message about generator start error
    pub fn sms_start_generator_err() -> Option<String> {
        let mut string_connection =
            String::from(sms_gateway_string_connection().unwrap_or_default());
        string_connection.push_str("Сбой+питания+от+электросети.+Сбой+старта+генератора.");
        Some(string_connection)
    }

    /// Text of Telegram-message about generator start error
    pub fn _tg_msg_start_generator_err() -> Option<String> {
        Some(String::from("Сбой питания от электросети.\nСбой старта генератора."))
    }

    /// The text of the SMS-message about the resumption of power supply from the network and the serviceability of the generator
    pub fn sms_power_restored_generator_ok() -> Option<String> {
        let mut string_connection =
            String::from(sms_gateway_string_connection().unwrap_or_default());
        string_connection.push_str(
            "Питание+от+электросети+восстановлено.+Генератор+исправен.+Генератор+работает.",
        );
        Some(string_connection)
    }

    /// The text of the Telegram-message about the resumption of power supply from the network and the serviceability of the generator
    pub fn _tg_msg_power_restored_generator_ok() -> Option<String> {
        Some(String::from(
            "Питание от электросети восстановлено.\nГенератор исправен.\nГенератор работает.",
        ))
    }

    /// The text of the SMS-message about the resumption of power supply from the network and the failure of the generator
    pub fn sms_power_restored_generator_err() -> Option<String> {
        let mut string_connection =
            String::from(sms_gateway_string_connection().unwrap_or_default());
        string_connection.push_str(
            "Питание+от+электросети+восстановлено.+Генератор+неисправен.+Генератор+не+работает.",
        );
        Some(string_connection)
    }

    /// The text of the Telegram-message about the resumption of power supply from the network and the failure of the generator
    pub fn _tg_msg_power_restored_generator_err() -> Option<String> {
        Some(String::from(
            "Питание от электросети восстановлено.\nГенератор неисправен.\nГенератор не работает.",
        ))
    }
}
