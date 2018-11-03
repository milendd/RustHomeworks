/// Необходимо е CodeIdentifier да имплементира Debug, за да можем да го
/// използваме в тестови assertion-и.
///
#[derive(Debug)]
pub struct CodeIdentifier {
    // Каквито полета ви трябват
    data: Vec<String>
}

impl CodeIdentifier {

    /// Функцията ще върне Option<CodeIdentifier>, което ще бъде:
    /// - None: ако входа не е валиден идентификатор. Вижте горе за това
    ///   какво значи "валиден идентификатор".
    /// - Some(code_identifier): Ако входа е валиден.
    ///
    pub fn new(identifier: &str) -> Option<Self> {
        let parsed = Self::parse_string(identifier.to_string());
        if parsed.len() == 0 {
            return None;
        }
        Some(CodeIdentifier { data: parsed })
    }

    /// Конвертира идентификатора до camelcased вариант.
    /// - Примерен вход: "some_var"
    /// - Примерен изход: "someVar"
    ///
    pub fn camelcase(&self) -> String {
        let res = Self::titlecase(&self);
        let first = res.chars().next().unwrap().to_string().to_lowercase();
        let last: String = res.chars().skip(1).collect();
        [first, last].join("")
    }

    /// Конвертира идентификатора до titlecased вариант (camelcased с първа заглавна буква).
    /// - Примерен вход: "some_var"
    /// - Примерен изход: "SomeVar"
    ///
    pub fn titlecase(&self) -> String {
        let v = &self.data;
        let result: Vec<String> = v.iter()
            .map(|x| 
                // first to upper and the rest
                [x.chars().next().unwrap().to_string().to_uppercase(), x.chars().skip(1).collect()]
                .join("")
            )
            .collect();
        
        result.join("")
    }

    /// Конвертира идентификатора до kebabcased вариант.
    /// - Примерен вход: "some_var"
    /// - Примерен изход: "some-var"
    ///
    pub fn kebabcase(&self) -> String {
        let v = &self.data;        
        v.join("-")
    }

    /// Конвертира идентификатора до underscored вариант.
    /// - Примерен вход: "some_var"
    /// - Примерен изход: "some_var"
    ///
    /// - Примерен вход: "Some_Var"
    /// - Примерен изход: "some_var"
    ///
    pub fn underscore(&self) -> String {
        let v = &self.data;        
        v.join("_")
    }

    /// Конвертира идентификатора до screaming-snakecased вариант.
    /// - Примерен вход: "some_var"
    /// - Примерен изход: "SOME_VAR"
    ///
    pub fn screaming_snakecase(&self) -> String {
        let v = &self.data;
        let result: Vec<String> = v.iter()
            .map(|x| x.to_uppercase())
            .collect();
        
        result.join("_")
    }

    fn parse_string(s: String) -> Vec<String> {
        let trimed = s.trim();

        let first_char = trimed.chars().next().unwrap();
        if !char::is_alphabetic(first_char) {
            return vec![];
        }

        let non_empty_ref = &trimed;
        let result: Vec<&str> = non_empty_ref.split_terminator('_').collect();
        let filtered = result.iter()
            .map(|&x| x.to_string().to_lowercase())
            .filter(|x| x != "")
            .collect::<Vec<String>>();

        let allowed: Vec<String> = result.iter()
            .map(|&x| x.to_string().to_lowercase())
            .filter(|x| x != "" && Self::is_valid(x.to_string()))
            .collect::<Vec<String>>();

        if filtered.len() != allowed.len() {
            return vec![];
        }

        allowed
    }

    fn is_valid(identifier: String) -> bool {
        let char_vec: Vec<char> = identifier.chars().collect();
        let allowed_vec: Vec<bool> = char_vec.iter()
            .map(|&x| char::is_numeric(x) || char::is_alphabetic(x))
            .collect();

        !allowed_vec.contains(&false)
    }
}