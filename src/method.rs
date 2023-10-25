#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Method {
    Delete,
    Get,
    Post,
    Put,
}

impl std::fmt::Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let method = match self {
            Self::Delete => "DELETE",
            Self::Get => "GET",
            Self::Post => "POST",
            Self::Put => "PUT",
        };

        write!(f, "{}", method)
    }
}

impl std::str::FromStr for Method {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(Self::Get),
            "POST" => Ok(Self::Post),
            "PUT" => Ok(Self::Put),
            "DELETE" => Ok(Self::Delete),
            method => Err(crate::Error::MethodConversion(String::from(method))),
        }
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_methods_to_str() {
        assert_eq!("GET", Method::Get.to_string());
        assert_eq!("POST", Method::Post.to_string());
        assert_eq!("PUT", Method::Put.to_string());
        assert_eq!("DELETE", Method::Delete.to_string());
    }

    #[test]
    fn test_methods_from_str() {
        assert_eq!(Method::from_str("GET"), Ok(Method::Get));
        assert_eq!(Method::from_str("POST"), Ok(Method::Post));
        assert_eq!(Method::from_str("PUT"), Ok(Method::Put));
        assert_eq!(Method::from_str("DELETE"), Ok(Method::Delete));
        let Err(crate::Error::MethodConversion(_)) = Method::from_str("Get") else {
            panic!()
        };
    }
}
