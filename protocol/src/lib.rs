pub mod dashboard;

use bigdecimal::BigDecimal;
use serde_derive::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Action {
    id: Id,
    kind: Kind,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Kind {
    Click,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Reaction {
    Scene(Scene),
    Delta(Delta),
}

pub type OverlayId = Option<Id>;

impl Reaction {
    pub fn overlay_id(&self) -> OverlayId {
        match self {
            Reaction::Scene(_) => None,
            Reaction::Delta(delta) => Some(delta.id.clone()),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Scene {
    Spinner,
    FullScreen(Layout),
    Dashboard(dashboard::Dashboard),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Footer {
    pub copyright: Value,
    pub menu: Menu,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Menu {
    pub items: Vec<MenuItem>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MenuItem {
    pub caption: Value,
}

/// Like `Layout`, but has physical appearance
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Container {
    Blank,
    Tabs(Vec<Tab>),
    Panel(Panel),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Panel {
    pub title: Option<Value>,
    pub body: Layout,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Tab {
    title: Value,
    body: Layout,
}

/// Like `Container`, but without physical appearance (row, column, center)
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Layout {
    Blank,
    Welcome,
    Bind(Bind),
    Control(Control),
    Row(Vec<Layout>),
    Column(Vec<Layout>),
    List(List),
    Container(Box<Container>),
}

impl From<Bind> for Layout {
    fn from(bind: Bind) -> Self {
        Self::Bind(bind)
    }
}

impl From<Control> for Layout {
    fn from(control: Control) -> Self {
        Self::Control(control)
    }
}

impl From<Container> for Layout {
    fn from(container: Container) -> Self {
        Self::Container(Box::new(container))
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct List {
    pub items: Vec<ListItem>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ListItem {
    pub title: Value,
    pub description: Value,
    pub bind: Bind,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Bind {
    Dynamic(Id),
    Fixed(Value),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Control {
    Button(Id),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Delta {
    pub id: Id,
    pub value: Value,
}

impl From<(Id, Value)> for Delta {
    fn from((id, value): (Id, Value)) -> Self {
        Self { id, value }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Id(String);

impl<T: AsRef<str>> From<T> for Id {
    fn from(value: T) -> Self {
        Id(value.as_ref().to_string())
    }
}

impl Default for Id {
    fn default() -> Self {
        Id("<default>".into())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Value {
    Nothing,
    String(String),
    Decimal(BigDecimal),
}

impl Default for Value {
    fn default() -> Self {
        Value::Nothing
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Nothing => write!(f, ""),
            Value::String(value) => write!(f, "{}", value),
            Value::Decimal(value) => write!(f, "{}", value),
        }
    }
}

macro_rules! value_convert {
    (@declare $var:ident $type:ty) => {
        impl From<$type> for Value {
            fn from(value: $type) -> Self {
                Value::$var(value.into())
            }
        }
    };
    ($var:ident : $($type:ty),*) => {
        $( value_convert!(@declare $var $type); )+
    };
}

value_convert!(Decimal: u8, i8, u16, i16, u32, i32, u64, i64, BigDecimal);

value_convert!(String: &str);
