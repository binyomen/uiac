use {
    crate::{UiacError, UiacResult},
    std::fmt,
    windows::Win32::{
        System::Com::{CoCreateInstance, CLSCTX_ALL, VARENUM, VARIANT, VT_BSTR, VT_I4},
        UI::Accessibility::{
            CUIAutomation, IUIAutomation, IUIAutomationCondition, IUIAutomationElement,
            IUIAutomationTreeWalker, UIA_AppBarControlTypeId, UIA_ButtonControlTypeId,
            UIA_CalendarControlTypeId, UIA_CheckBoxControlTypeId, UIA_ComboBoxControlTypeId,
            UIA_CustomControlTypeId, UIA_DataGridControlTypeId, UIA_DataItemControlTypeId,
            UIA_DocumentControlTypeId, UIA_EditControlTypeId, UIA_GroupControlTypeId,
            UIA_HeaderControlTypeId, UIA_HeaderItemControlTypeId, UIA_HyperlinkControlTypeId,
            UIA_ImageControlTypeId, UIA_ListControlTypeId, UIA_ListItemControlTypeId,
            UIA_MenuBarControlTypeId, UIA_MenuControlTypeId, UIA_MenuItemControlTypeId,
            UIA_PaneControlTypeId, UIA_ProgressBarControlTypeId, UIA_RadioButtonControlTypeId,
            UIA_ScrollBarControlTypeId, UIA_SemanticZoomControlTypeId, UIA_SeparatorControlTypeId,
            UIA_SliderControlTypeId, UIA_SpinnerControlTypeId, UIA_SplitButtonControlTypeId,
            UIA_StatusBarControlTypeId, UIA_TabControlTypeId, UIA_TabItemControlTypeId,
            UIA_TableControlTypeId, UIA_TextControlTypeId, UIA_ThumbControlTypeId,
            UIA_TitleBarControlTypeId, UIA_ToolBarControlTypeId, UIA_ToolTipControlTypeId,
            UIA_TreeControlTypeId, UIA_TreeItemControlTypeId, UIA_WindowControlTypeId,
            UIA_CONTROLTYPE_ID, UIA_PROPERTY_ID,
        },
    },
};

macro_rules! wrapper_fn {
    ($fn_name:ident, $inner_fn_name:ident, $result_type:ident $(, $arg:ident: $arg_type:ty),*) => {
        pub fn $fn_name(&self$(, $arg: &$arg_type),*) -> $crate::UiacResult<$result_type> {
            std::result::Result::Ok($result_type {
                inner: unsafe { self.inner.$inner_fn_name($(&$arg.inner),*) }?,
            })
        }
    };
}

macro_rules! wrapper_option_fn {
    ($fn_name:ident, $inner_fn_name:ident, $result_type:ident $(, $arg:ident: $arg_type:ty),*) => {
        pub fn $fn_name(&self$(, $arg: &$arg_type),*) -> $crate::UiacResult<std::option::Option<$result_type>> {
            let inner = unsafe { $crate::opt_result(self.inner.$inner_fn_name($(&$arg.inner),*)) }?;
            Ok(match inner {
                std::option::Option::Some(inner) => std::option::Option::Some($result_type {inner}),
                std::option::Option::None => std::option::Option::None,
            })
        }
    };
}

pub struct Automation {
    inner: IUIAutomation,
}

impl Automation {
    pub fn new() -> UiacResult<Self> {
        Ok(Self {
            inner: unsafe {
                CoCreateInstance(&CUIAutomation, None /*pUnkOuter*/, CLSCTX_ALL)
            }?,
        })
    }

    wrapper_fn!(get_root_element, GetRootElement, Element);
    wrapper_fn!(
        create_tree_walker,
        CreateTreeWalker,
        TreeWalker,
        condition: Condition
    );
    wrapper_fn!(create_true_condition, CreateTrueCondition, Condition);
}

pub struct Element {
    inner: IUIAutomationElement,
}

impl Element {
    pub fn get_current_property_value(&self, property_id: UIA_PROPERTY_ID) -> UiacResult<Variant> {
        Ok(Variant {
            inner: unsafe { self.inner.GetCurrentPropertyValue(property_id.0 as i32) }?,
        })
    }
}

pub struct TreeWalker {
    inner: IUIAutomationTreeWalker,
}

impl TreeWalker {
    wrapper_option_fn!(
        get_first_child_element,
        GetFirstChildElement,
        Element,
        element: Element
    );
    wrapper_option_fn!(
        get_next_sibling_element,
        GetNextSiblingElement,
        Element,
        element: Element
    );
}

pub struct Condition {
    inner: IUIAutomationCondition,
}

pub struct Variant {
    inner: VARIANT,
}

impl Variant {
    pub fn vt(&self) -> VARENUM {
        unsafe { self.inner.Anonymous.Anonymous.vt }
    }

    pub fn as_string(&self) -> UiacResult<String> {
        if self.vt() != VT_BSTR {
            return Err(UiacError::InvalidVariantType);
        }

        let bstr = unsafe { &self.inner.Anonymous.Anonymous.Anonymous.bstrVal };
        Ok((&**bstr).try_into()?)
    }

    pub fn as_control_type(&self) -> UiacResult<ControlType> {
        if self.vt() != VT_I4 {
            return Err(UiacError::InvalidVariantType);
        }

        let l_val = unsafe { self.inner.Anonymous.Anonymous.Anonymous.lVal };
        Ok(ControlType::new(UIA_CONTROLTYPE_ID(l_val as u32)))
    }
}

pub enum ControlType {
    AppBar,
    Button,
    Calendar,
    CheckBox,
    ComboBox,
    Custom,
    DataGrid,
    DataItem,
    Document,
    Edit,
    Group,
    Header,
    HeaderItem,
    Hyperlink,
    Image,
    List,
    ListItem,
    MenuBar,
    Menu,
    MenuItem,
    Pane,
    ProgressBar,
    RadioButton,
    ScrollBar,
    SemanticZoom,
    Separator,
    Slider,
    Spinner,
    SplitButton,
    StatusBar,
    Tab,
    TabItem,
    Table,
    Text,
    Thumb,
    TitleBar,
    ToolBar,
    ToolTip,
    Tree,
    TreeItem,
    Window,
}

impl ControlType {
    fn new(id: UIA_CONTROLTYPE_ID) -> Self {
        if id == UIA_AppBarControlTypeId {
            ControlType::AppBar
        } else if id == UIA_ButtonControlTypeId {
            ControlType::Button
        } else if id == UIA_CalendarControlTypeId {
            ControlType::Calendar
        } else if id == UIA_CheckBoxControlTypeId {
            ControlType::CheckBox
        } else if id == UIA_ComboBoxControlTypeId {
            ControlType::ComboBox
        } else if id == UIA_CustomControlTypeId {
            ControlType::Custom
        } else if id == UIA_DataGridControlTypeId {
            ControlType::DataGrid
        } else if id == UIA_DataItemControlTypeId {
            ControlType::DataItem
        } else if id == UIA_DocumentControlTypeId {
            ControlType::Document
        } else if id == UIA_EditControlTypeId {
            ControlType::Edit
        } else if id == UIA_GroupControlTypeId {
            ControlType::Group
        } else if id == UIA_HeaderControlTypeId {
            ControlType::Header
        } else if id == UIA_HeaderItemControlTypeId {
            ControlType::HeaderItem
        } else if id == UIA_HyperlinkControlTypeId {
            ControlType::Hyperlink
        } else if id == UIA_ImageControlTypeId {
            ControlType::Image
        } else if id == UIA_ListControlTypeId {
            ControlType::List
        } else if id == UIA_ListItemControlTypeId {
            ControlType::ListItem
        } else if id == UIA_MenuBarControlTypeId {
            ControlType::MenuBar
        } else if id == UIA_MenuControlTypeId {
            ControlType::Menu
        } else if id == UIA_MenuItemControlTypeId {
            ControlType::MenuItem
        } else if id == UIA_PaneControlTypeId {
            ControlType::Pane
        } else if id == UIA_ProgressBarControlTypeId {
            ControlType::ProgressBar
        } else if id == UIA_RadioButtonControlTypeId {
            ControlType::RadioButton
        } else if id == UIA_ScrollBarControlTypeId {
            ControlType::ScrollBar
        } else if id == UIA_SemanticZoomControlTypeId {
            ControlType::SemanticZoom
        } else if id == UIA_SeparatorControlTypeId {
            ControlType::Separator
        } else if id == UIA_SliderControlTypeId {
            ControlType::Slider
        } else if id == UIA_SpinnerControlTypeId {
            ControlType::Spinner
        } else if id == UIA_SplitButtonControlTypeId {
            ControlType::SplitButton
        } else if id == UIA_StatusBarControlTypeId {
            ControlType::StatusBar
        } else if id == UIA_TabControlTypeId {
            ControlType::Tab
        } else if id == UIA_TabItemControlTypeId {
            ControlType::TabItem
        } else if id == UIA_TableControlTypeId {
            ControlType::Table
        } else if id == UIA_TextControlTypeId {
            ControlType::Text
        } else if id == UIA_ThumbControlTypeId {
            ControlType::Thumb
        } else if id == UIA_TitleBarControlTypeId {
            ControlType::TitleBar
        } else if id == UIA_ToolBarControlTypeId {
            ControlType::ToolBar
        } else if id == UIA_ToolTipControlTypeId {
            ControlType::ToolTip
        } else if id == UIA_TreeControlTypeId {
            ControlType::Tree
        } else if id == UIA_TreeItemControlTypeId {
            ControlType::TreeItem
        } else if id == UIA_WindowControlTypeId {
            ControlType::Window
        } else {
            panic!("invalid control type id {}", id.0);
        }
    }
}

impl fmt::Display for ControlType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ControlType::AppBar => write!(f, "AppBar"),
            ControlType::Button => write!(f, "Button"),
            ControlType::Calendar => write!(f, "Calendar"),
            ControlType::CheckBox => write!(f, "CheckBox"),
            ControlType::ComboBox => write!(f, "ComboBox"),
            ControlType::Custom => write!(f, "Custom"),
            ControlType::DataGrid => write!(f, "DataGrid"),
            ControlType::DataItem => write!(f, "DataItem"),
            ControlType::Document => write!(f, "Document"),
            ControlType::Edit => write!(f, "Edit"),
            ControlType::Group => write!(f, "Group"),
            ControlType::Header => write!(f, "Header"),
            ControlType::HeaderItem => write!(f, "HeaderItem"),
            ControlType::Hyperlink => write!(f, "Hyperlink"),
            ControlType::Image => write!(f, "Image"),
            ControlType::List => write!(f, "List"),
            ControlType::ListItem => write!(f, "ListItem"),
            ControlType::MenuBar => write!(f, "MenuBar"),
            ControlType::Menu => write!(f, "Menu"),
            ControlType::MenuItem => write!(f, "MenuItem"),
            ControlType::Pane => write!(f, "Pane"),
            ControlType::ProgressBar => write!(f, "ProgressBar"),
            ControlType::RadioButton => write!(f, "RadioButton"),
            ControlType::ScrollBar => write!(f, "ScrollBar"),
            ControlType::SemanticZoom => write!(f, "SemanticZoom"),
            ControlType::Separator => write!(f, "Separator"),
            ControlType::Slider => write!(f, "Slider"),
            ControlType::Spinner => write!(f, "Spinner"),
            ControlType::SplitButton => write!(f, "SplitButton"),
            ControlType::StatusBar => write!(f, "StatusBar"),
            ControlType::Tab => write!(f, "Tab"),
            ControlType::TabItem => write!(f, "TabItem"),
            ControlType::Table => write!(f, "Table"),
            ControlType::Text => write!(f, "Text"),
            ControlType::Thumb => write!(f, "Thumb"),
            ControlType::TitleBar => write!(f, "TitleBar"),
            ControlType::ToolBar => write!(f, "ToolBar"),
            ControlType::ToolTip => write!(f, "ToolTip"),
            ControlType::Tree => write!(f, "Tree"),
            ControlType::TreeItem => write!(f, "TreeItem"),
            ControlType::Window => write!(f, "Window"),
        }
    }
}
