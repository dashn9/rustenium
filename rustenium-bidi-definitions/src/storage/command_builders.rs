use super::commands::*;
impl GetCookies {
    pub fn builder() -> GetCookiesBuilder {
        <GetCookiesBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct GetCookiesBuilder {
    filter: Option<super::types::CookieFilter>,
    partition: Option<super::types::PartitionDescriptor>,
}
impl GetCookiesBuilder {
    pub fn filter(mut self, filter: impl Into<super::types::CookieFilter>) -> Self {
        self.filter = Some(filter.into());
        self
    }
    pub fn partition(mut self, partition: impl Into<super::types::PartitionDescriptor>) -> Self {
        self.partition = Some(partition.into());
        self
    }
    pub fn build(self) -> GetCookies {
        GetCookies {
            method: GetCookiesMethod::GetCookies,
            params: GetCookiesParams {
                filter: self.filter,
                partition: self.partition,
            },
        }
    }
}
impl SetCookie {
    pub fn builder() -> SetCookieBuilder {
        <SetCookieBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct SetCookieBuilder {
    cookie: Option<super::types::PartialCookie>,
    partition: Option<super::types::PartitionDescriptor>,
}
impl SetCookieBuilder {
    pub fn cookie(mut self, cookie: impl Into<super::types::PartialCookie>) -> Self {
        self.cookie = Some(cookie.into());
        self
    }
    pub fn partition(mut self, partition: impl Into<super::types::PartitionDescriptor>) -> Self {
        self.partition = Some(partition.into());
        self
    }
    pub fn build(self) -> Result<SetCookie, String> {
        Ok(SetCookie {
            method: SetCookieMethod::SetCookie,
            params: SetCookieParams {
                cookie: self
                    .cookie
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(cookie)))?,
                partition: self.partition,
            },
        })
    }
}
impl DeleteCookies {
    pub fn builder() -> DeleteCookiesBuilder {
        <DeleteCookiesBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct DeleteCookiesBuilder {
    filter: Option<super::types::CookieFilter>,
    partition: Option<super::types::PartitionDescriptor>,
}
impl DeleteCookiesBuilder {
    pub fn filter(mut self, filter: impl Into<super::types::CookieFilter>) -> Self {
        self.filter = Some(filter.into());
        self
    }
    pub fn partition(mut self, partition: impl Into<super::types::PartitionDescriptor>) -> Self {
        self.partition = Some(partition.into());
        self
    }
    pub fn build(self) -> DeleteCookies {
        DeleteCookies {
            method: DeleteCookiesMethod::DeleteCookies,
            params: DeleteCookiesParams {
                filter: self.filter,
                partition: self.partition,
            },
        }
    }
}
