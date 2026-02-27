use std::{borrow::Cow, collections::HashMap, sync::LazyLock};

pub struct ModCommand {

}

pub struct ModEvent {}

pub struct ModType {}

pub struct Module<'a> {
    name: Cow<'a, str>,
    commands: Vec<ModCommand>,
    events: Vec<ModEvent>,
    types: Vec<ModType>
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TokenId(usize);

pub struct Token {
    name: String
}

pub struct TokenBuilder {
    pub name: String
}

impl TokenBuilder {
    pub fn name(mut self, name: String) -> Self {
        self.name = name;
        self
    }
    pub fn build (self) -> Token {
        Token { name: self.name }
    }
}

impl Default for TokenBuilder {
    fn default() -> Self {
        TokenBuilder { name: String::new() }
    }
}

pub struct TokenStore {
    tokens: Vec<Token>,
    // or indexed by ID for fast lookup
    index: HashMap<TokenId, usize>,
}

impl TokenStore {
    pub fn new() -> Self {
        Self {
            tokens: Vec::new(),
            index: HashMap::new(),
        }
    }
    pub fn insert(&mut self, token: Token) -> TokenId {
        let id = TokenId(self.tokens.len());
        self.tokens.push(token);
        id
    }

    pub fn get(&self, id: TokenId) -> &Token {
        &self.tokens[id.0]
    }
}

pub struct Property {
    name: Token,
    value: Token,
}

pub enum TypeDef {
    Type {

    },
    Enum {
        name: Token
    },
    Struct {
        properties: Vec<Property>
    }
}

static TOKEN_STORE: LazyLock<TokenStore> = LazyLock::new(|| TokenStore::new());