use std::str::FromStr;

use crate::errors::Error;

#[derive(Clone, Debug, PartialEq)]
pub struct Pod {
    pub name: String,
    pub ready: String,
    pub status: String,
    pub restarts: String,
    pub age: String,
}

impl FromStr for Pod {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split_whitespace().collect();

        if parts.len() != 5 && parts.len() != 7 {
            println!("{:?}", parts);
            return Err(Error::ParseOutputError);
        };

        // Should be a number of restarts
        if parts[3].parse::<u16>().is_err() {
            return Err(Error::ParseOutputError);
        }

        let restarts;
        let age;

        if parts.len() == 7 {
            // ex: 1 (8d ago)
            restarts = parts[3..=5].join(" ");
            age = parts[6].to_string();
        } else {
            // ex: 0
            restarts = parts[3].to_string();
            age = parts[4].to_string();
        }

        let name = parts[0].to_string();
        let ready = parts[1].to_string();
        let status = parts[2].to_string();

        Ok(Pod {
            name,
            ready,
            status,
            restarts,
            age,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::errors::Error;

    use super::Pod;

    #[test]
    fn parse_pod_info() {
        //given
        let pod_info =
            "wiremock-docker-64962f5dh4-scq24               1/1     Running   0              18d";
        let expected = Pod {
            name: "wiremock-docker-64962f5dh4-scq24".to_string(),
            ready: "1/1".to_string(),
            status: "Running".to_string(),
            restarts: "0".to_string(),
            age: "18d".to_string(),
        };

        // when
        let actual: Pod = pod_info.parse().unwrap();

        //then
        assert_eq!(actual, expected);
    }
    #[test]
    fn parse_pod_info_with_restart_date() {
        //given
        let pod_info =
            "some-7dd394858c-sxctv                       1/1     Running   1 (8d ago)     30d";
        let expected = Pod {
            name: "some-7dd394858c-sxctv".to_string(),
            ready: "1/1".to_string(),
            status: "Running".to_string(),
            restarts: "1 (8d ago)".to_string(),
            age: "30d".to_string(),
        };

        // when
        let actual: Pod = pod_info.parse().unwrap();

        //then
        assert_eq!(actual, expected);
    }

    #[test]
    fn should_not_parse_headers() {
        //given
        let header =
            "NAME                                           READY   STATUS    RESTARTS       AGE";
        //when
        let result: Result<Pod, Error> = header.parse();
        // then expect error
        result.unwrap_err();
    }
}
