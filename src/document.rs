use chrono::NaiveDate;

/// Travel document
#[derive(Debug, PartialEq, Clone)]
pub enum Document {
    /// Passport
    Passport(Passport),
}

/// Gender
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Gender {
    /// Male
    Male,
    /// Female
    Female,
    /// Other/unspecified
    Other,
}

/// Passport
#[derive(Debug, PartialEq, Clone)]
pub struct Passport {
    /// Country (ISO 3166-1 code)
    pub country: String,
    /// Surname
    pub surname: String,
    /// Given names
    pub given_names: Vec<String>,
    /// Passport number
    pub passport_number: String,
    /// Nationality (ISO 3166-1 code)
    pub nationality: String,
    /// Date of birth
    pub birth_date: NaiveDate,
    /// Gender
    pub gender: Gender,
    /// Date of expiry
    pub expiry_date: NaiveDate,
}
