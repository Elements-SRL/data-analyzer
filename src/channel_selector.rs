// #[derive(Clone, Copy)]
pub enum ChannelSelector {
    All,
    Channel(usize),
}

pub enum ErrorKind {
    FromStr,
}

impl ChannelSelector {
    pub fn from_str(line: &str) -> Result<Self, ErrorKind> {
        let line = line.trim();
        match line {
            "all" | "a"   => {
                Ok(Self::All)
            },
            _ => match line.parse(){
                Ok(l_usize) => {
                    Ok(Self::Channel(l_usize))
                },
                Err(e) => {
                    Err(ErrorKind::FromStr)
                }
            }
        }
    }
}