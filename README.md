# uiac

A command line interface for UIA on Windows.

## Commands

### `dump`

Dumps the UIA tree of the current desktop.

```powershell
PS> uiac dump
│ Name = "Desktop 1", ControlType = Pane
  │ Name = "Taskbar", ControlType = Pane
    │ Name = "Start", ControlType = Button
    │ Name = [no name], ControlType = Pane
      │ Name = "Running applications", ControlType = Pane
        │ Name = "Running applications", ControlType = ToolBar
          │ Name = "Firefox", ControlType = Button
          │ Name = [no name], ControlType = Custom
    │ Name = [no name], ControlType = Pane
      │ Name = "Notification Chevron", ControlType = Button
      │ Name = [no name], ControlType = Pane
        │ Name = "User Promoted Notification Area", ControlType = ToolBar
          │ Name = "Meet Now", ControlType = Button
          │ Name = "28 min to full charge", ControlType = Button
          │ Name = "my wifi\nInternet access", ControlType = Button
          │ Name = "Speakers / Headphones: 58%", ControlType = Button
      │ Name = "Windows Ink Workspace", ControlType = Button
      │ Name = "Tray Input Indicator", ControlType = Pane
        │ Name = "Tray Input Indicator - English (United States) - United States-Dvorak keyboard", ControlType = Button
      │ Name = "System Clock, 1:04 AM, \u{200e}1/\u{200e}22/\u{200e}2023", ControlType = Button
      │ Name = "Action Center, No new notifications (Focus assist on)", ControlType = Button
      │ Name = "Show desktop", ControlType = Button
  │ Name = [no name], ControlType = Window
    │ Name = [no name], ControlType = TitleBar
  │ Name = "Program Manager", ControlType = Pane
    │ Name = [no name], ControlType = Pane
      │ Name = "Desktop", ControlType = List
        │ Name = "Recycle Bin", ControlType = ListItem
```
