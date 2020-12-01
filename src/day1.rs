use crate::Err;

pub fn first(input: &[u32]) -> Result<u32, Err> {
    for o in input {
        for i in input {
            if o + i == 2020 {
                return Ok(o * i);
            }
        }
    }

    Err(Err::Failed())
}

pub fn second(account: &[u32]) -> Result<u32, Err> {
    for o in account {
        for i in account {
            for l in account {
                if o + i + l == 2020 {
                    return Ok(o * i * l);
                }
            }
        }
    }

    Err(Err::Failed())
}
