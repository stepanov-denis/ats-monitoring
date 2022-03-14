pub mod gateway {
    pub fn sms_gateway_string_connection() -> String {
        let string_connection = String::from("https://api-mapper.clicksend.com/http/v2/send.php?method=http&username=development-service@yandex.ru&key=1E82A334-89D8-985C-526B-712DB70A713D&to=+79139402913&message=");
        string_connection
    }

    pub fn sms_generator_work_err() -> String {
        let mut string_connection = String::from(sms_gateway_string_connection());
        string_connection
            .push_str("Авария!+Генератор+неисправен!+Срочно+произведите+сервисные+работы!");
        string_connection
    }

    pub fn sms_generator_work_restored() -> String {
        let mut string_connection = String::from(sms_gateway_string_connection());
        string_connection.push_str("Работоспособность+генератора+в+режиме+трансляции+питания+от+электросети+восстановлена.+Генератор+исправен.+Генератор+работает.");
        string_connection
    }

    pub fn sms_start_generator_ok() -> String {
        let mut string_connection = String::from(sms_gateway_string_connection());
        string_connection.push_str("Сбой+питания+от+электросети.+Успешный+старт+генератора.");
        string_connection
    }

    pub fn sms_start_generator_err() -> String {
        let mut string_connection = String::from(sms_gateway_string_connection());
        string_connection.push_str("Сбой+питания+от+электросети.+Сбой+старта+генератора.");
        string_connection
    }

    pub fn sms_power_restored_generator_ok() -> String {
        let mut string_connection = String::from(sms_gateway_string_connection());
        string_connection.push_str(
            "Питание+от+электросети+восстановлено.+Генератор+исправен.+Генератор+работает.",
        );
        string_connection
    }

    pub fn sms_power_restored_generator_err() -> String {
        let mut string_connection = String::from(sms_gateway_string_connection());
        string_connection.push_str(
            "Питание+от+электросети+восстановлено.+Генератор+неисправен.+Генератор+не+работает.",
        );
        string_connection
    }
}
