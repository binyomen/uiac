use {
    crate::{
        wrappers::{Automation, Element, TreeWalker},
        UiacResult,
    },
    windows::Win32::UI::Accessibility::{UIA_ControlTypePropertyId, UIA_NamePropertyId},
};

pub fn dump() -> UiacResult<()> {
    let automation = Automation::new()?;
    let walker = automation.create_tree_walker(&automation.create_true_condition()?)?;
    let root = automation.get_root_element()?;

    dump_recursive(&walker, &root, 0)
}

fn dump_recursive(walker: &TreeWalker, element: &Element, indent_level: usize) -> UiacResult<()> {
    print_element(element, indent_level)?;

    let mut child_option = walker.get_first_child_element(element)?;
    while let Some(child) = child_option {
        dump_recursive(walker, &child, indent_level + 1)?;
        child_option = walker.get_next_sibling_element(&child)?;
    }

    Ok(())
}

fn print_element(element: &Element, indent_level: usize) -> UiacResult<()> {
    print!("{}│ ", "  ".repeat(indent_level));

    let name = element
        .get_current_property_value(UIA_NamePropertyId)?
        .as_string()?;
    let name = if name.is_empty() {
        "[no name]".to_owned()
    } else {
        format!("{name:?}")
    };

    let control_type = element
        .get_current_property_value(UIA_ControlTypePropertyId)?
        .as_control_type()?;

    println!("Name = {name}, ControlType = {control_type}");

    Ok(())
}
