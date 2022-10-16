use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use strum_macros::EnumIter;

/// Valid ways how `Reedline::read_line()` can return
#[derive(Debug)]
pub enum Signal {
    /// Entry succeeded with the provided content
    Success(String),
    /// Entry was aborted with `Ctrl+C`
    CtrlC, // Interrupt current editing
    /// Abort with `Ctrl+D` signalling `EOF` or abort of a whole interactive session
    CtrlD, // End terminal session
}

/// Editing actions which can be mapped to key bindings.
///
/// Executed by `Reedline::run_edit_commands()`
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq, EnumIter)]
pub enum EditCommand {
    /// Move to the start of the buffer
    MoveToStart,

    /// Move to the start of the current line
    MoveToLineStart,

    /// Move to the end of the buffer
    MoveToEnd,

    /// Move to the end of the current line
    MoveToLineEnd,

    /// Move one character to the left
    MoveLeft,

    /// Move one character to the right
    MoveRight,

    /// Move one word to the left
    MoveWordLeft,

    /// Move one WORD to the left
    MoveBigWordLeft,

    /// Move one word to the right
    MoveWordRight,

    /// Move one word to the right, stop at start of word
    MoveWordRightStart,

    /// Move one WORD to the right, stop at start of WORD
    MoveBigWordRightStart,

    /// Move one word to the right, stop at end of word
    MoveWordRightEnd,

    /// Move one WORD to the right, stop at end of WORD
    MoveBigWordRightEnd,

    /// Move to position
    MoveToPosition(usize),

    /// Insert a character at the current insertion point
    InsertChar(char),

    /// Insert a string at the current insertion point
    InsertString(String),

    /// Inserts the system specific new line character
    ///
    /// - On Unix systems LF (`"\n"`)
    /// - On Windows CRLF (`"\r\n"`)
    InsertNewline,

    /// Replace a character
    ReplaceChar(char),

    /// Replace characters with string
    ReplaceChars(usize, String),

    /// Backspace delete from the current insertion point
    Backspace,

    /// Delete in-place from the current insertion point
    Delete,

    /// Cut the grapheme right from the current insertion point
    CutChar,

    /// Backspace delete a word from the current insertion point
    BackspaceWord,

    /// Delete in-place a word from the current insertion point
    DeleteWord,

    /// Clear the current buffer
    Clear,

    /// Clear to the end of the current line
    ClearToLineEnd,

    /// Cut the current line
    CutCurrentLine,

    /// Cut from the start of the buffer to the insertion point
    CutFromStart,

    /// Cut from the start of the current line to the insertion point
    CutFromLineStart,

    /// Cut from the insertion point to the end of the buffer
    CutToEnd,

    /// Cut from the insertion point to the end of the current line
    CutToLineEnd,

    /// Cut the word left of the insertion point
    CutWordLeft,

    /// Cut the WORD left of the insertion point
    CutBigWordLeft,

    /// Cut the word right of the insertion point
    CutWordRight,

    /// Cut the word right of the insertion point
    CutBigWordRight,

    /// Cut the word right of the insertion point and any following space
    CutWordRightToNext,

    /// Cut the WORD right of the insertion point and any following space
    CutBigWordRightToNext,

    /// Paste the cut buffer in front of the insertion point (Emacs, vi `P`)
    PasteCutBufferBefore,

    /// Paste the cut buffer in front of the insertion point (vi `p`)
    PasteCutBufferAfter,

    /// Upper case the current word
    UppercaseWord,

    /// Lower case the current word
    LowercaseWord,

    /// Capitalize the current character
    CapitalizeChar,

    /// Switch the case of the current character
    SwitchcaseChar,

    /// Swap the current word with the word to the right
    SwapWords,

    /// Swap the current grapheme/character with the one to the right
    SwapGraphemes,

    /// Undo the previous edit command
    Undo,

    /// Redo an edit command from the undo history
    Redo,

    /// CutUntil right until char
    CutRightUntil(char),

    /// CutUntil right before char
    CutRightBefore(char),

    /// CutUntil right until char
    MoveRightUntil(char),

    /// CutUntil right before char
    MoveRightBefore(char),

    /// CutUntil left until char
    CutLeftUntil(char),

    /// CutUntil left before char
    CutLeftBefore(char),

    /// CutUntil left until char
    MoveLeftUntil(char),

    /// CutUntil left before char
    MoveLeftBefore(char),
}

impl Display for EditCommand {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            EditCommand::MoveToStart => write!(f, "MoveToStart"),
            EditCommand::MoveToLineStart => write!(f, "MoveToLineStart"),
            EditCommand::MoveToEnd => write!(f, "MoveToEnd"),
            EditCommand::MoveToLineEnd => write!(f, "MoveToLineEnd"),
            EditCommand::MoveLeft => write!(f, "MoveLeft"),
            EditCommand::MoveRight => write!(f, "MoveRight"),
            EditCommand::MoveWordLeft => write!(f, "MoveWordLeft"),
            EditCommand::MoveBigWordLeft => write!(f, "MoveBigWordLeft"),
            EditCommand::MoveWordRight => write!(f, "MoveWordRight"),
            EditCommand::MoveWordRightEnd => write!(f, "MoveWordRightEnd"),
            EditCommand::MoveBigWordRightEnd => write!(f, "MoveBigWordRightEnd"),
            EditCommand::MoveWordRightStart => write!(f, "MoveWordRightStart"),
            EditCommand::MoveBigWordRightStart => write!(f, "MoveBigWordRightStart"),
            EditCommand::MoveToPosition(_) => write!(f, "MoveToPosition  Value: <int>"),
            EditCommand::InsertChar(_) => write!(f, "InsertChar  Value: <char>"),
            EditCommand::InsertString(_) => write!(f, "InsertString Value: <string>"),
            EditCommand::InsertNewline => write!(f, "InsertNewline"),
            EditCommand::ReplaceChar(_) => write!(f, "ReplaceChar <char>"),
            EditCommand::ReplaceChars(_, _) => write!(f, "ReplaceChars <int> <string>"),
            EditCommand::Backspace => write!(f, "Backspace"),
            EditCommand::Delete => write!(f, "Delete"),
            EditCommand::CutChar => write!(f, "CutChar"),
            EditCommand::BackspaceWord => write!(f, "BackspaceWord"),
            EditCommand::DeleteWord => write!(f, "DeleteWord"),
            EditCommand::Clear => write!(f, "Clear"),
            EditCommand::ClearToLineEnd => write!(f, "ClearToLineEnd"),
            EditCommand::CutCurrentLine => write!(f, "CutCurrentLine"),
            EditCommand::CutFromStart => write!(f, "CutFromStart"),
            EditCommand::CutFromLineStart => write!(f, "CutFromLineStart"),
            EditCommand::CutToEnd => write!(f, "CutToEnd"),
            EditCommand::CutToLineEnd => write!(f, "CutToLineEnd"),
            EditCommand::CutWordLeft => write!(f, "CutWordLeft"),
            EditCommand::CutBigWordLeft => write!(f, "CutBigWordLeft"),
            EditCommand::CutWordRight => write!(f, "CutWordRight"),
            EditCommand::CutBigWordRight => write!(f, "CutBigWordRight"),
            EditCommand::CutWordRightToNext => write!(f, "CutWordRightToNext"),
            EditCommand::CutBigWordRightToNext => write!(f, "CutBigWordRightToNext"),
            EditCommand::PasteCutBufferBefore => write!(f, "PasteCutBufferBefore"),
            EditCommand::PasteCutBufferAfter => write!(f, "PasteCutBufferAfter"),
            EditCommand::UppercaseWord => write!(f, "UppercaseWord"),
            EditCommand::LowercaseWord => write!(f, "LowercaseWord"),
            EditCommand::SwitchcaseChar => write!(f, "SwitchcaseChar"),
            EditCommand::CapitalizeChar => write!(f, "CapitalizeChar"),
            EditCommand::SwapWords => write!(f, "SwapWords"),
            EditCommand::SwapGraphemes => write!(f, "SwapGraphemes"),
            EditCommand::Undo => write!(f, "Undo"),
            EditCommand::Redo => write!(f, "Redo"),
            EditCommand::CutRightUntil(_) => write!(f, "CutRightUntil Value: <char>"),
            EditCommand::CutRightBefore(_) => write!(f, "CutRightBefore Value: <char>"),
            EditCommand::MoveRightUntil(_) => write!(f, "MoveRightUntil Value: <char>"),
            EditCommand::MoveRightBefore(_) => write!(f, "MoveRightBefore Value: <char>"),
            EditCommand::CutLeftUntil(_) => write!(f, "CutLeftUntil Value: <char>"),
            EditCommand::CutLeftBefore(_) => write!(f, "CutLeftBefore Value: <char>"),
            EditCommand::MoveLeftUntil(_) => write!(f, "MoveLeftUntil Value: <char>"),
            EditCommand::MoveLeftBefore(_) => write!(f, "MoveLeftBefore Value: <char>"),
        }
    }
}

impl EditCommand {
    /// Determine if a certain operation should be undoable
    /// or if the operations should be coalesced for undoing
    pub fn edit_type(&self) -> EditType {
        match self {
            // Cursor moves
            EditCommand::MoveToStart
            | EditCommand::MoveToEnd
            | EditCommand::MoveToLineStart
            | EditCommand::MoveToLineEnd
            | EditCommand::MoveToPosition(_)
            | EditCommand::MoveLeft
            | EditCommand::MoveRight
            | EditCommand::MoveWordLeft
            | EditCommand::MoveBigWordLeft
            | EditCommand::MoveWordRight
            | EditCommand::MoveWordRightStart
            | EditCommand::MoveBigWordRightStart
            | EditCommand::MoveWordRightEnd
            | EditCommand::MoveBigWordRightEnd
            | EditCommand::MoveRightUntil(_)
            | EditCommand::MoveRightBefore(_)
            | EditCommand::MoveLeftUntil(_)
            | EditCommand::MoveLeftBefore(_) => EditType::MoveCursor,

            // Text edits
            EditCommand::InsertChar(_)
            | EditCommand::Backspace
            | EditCommand::Delete
            | EditCommand::CutChar
            | EditCommand::InsertString(_)
            | EditCommand::InsertNewline
            | EditCommand::ReplaceChar(_)
            | EditCommand::ReplaceChars(_, _)
            | EditCommand::BackspaceWord
            | EditCommand::DeleteWord
            | EditCommand::Clear
            | EditCommand::ClearToLineEnd
            | EditCommand::CutCurrentLine
            | EditCommand::CutFromStart
            | EditCommand::CutFromLineStart
            | EditCommand::CutToLineEnd
            | EditCommand::CutToEnd
            | EditCommand::CutWordLeft
            | EditCommand::CutBigWordLeft
            | EditCommand::CutWordRight
            | EditCommand::CutBigWordRight
            | EditCommand::CutWordRightToNext
            | EditCommand::CutBigWordRightToNext
            | EditCommand::PasteCutBufferBefore
            | EditCommand::PasteCutBufferAfter
            | EditCommand::UppercaseWord
            | EditCommand::LowercaseWord
            | EditCommand::SwitchcaseChar
            | EditCommand::CapitalizeChar
            | EditCommand::SwapWords
            | EditCommand::SwapGraphemes
            | EditCommand::CutRightUntil(_)
            | EditCommand::CutRightBefore(_)
            | EditCommand::CutLeftUntil(_)
            | EditCommand::CutLeftBefore(_) => EditType::EditText,

            EditCommand::Undo | EditCommand::Redo => EditType::UndoRedo,
        }
    }
}

/// Specifies the types of edit commands, used to simplify grouping edits
/// to mark undo behavior
#[derive(PartialEq, Eq)]
pub enum EditType {
    /// Cursor movement commands
    MoveCursor,
    /// Undo/Redo commands
    UndoRedo,
    /// Text editing commands
    EditText,
}

/// Every line change should come with an `UndoBehavior` tag, which can be used to
/// calculate how the change should be reflected on the undo stack
#[derive(Debug)]
pub enum UndoBehavior {
    /// Character insertion, tracking the character inserted
    InsertCharacter(char),
    /// Backspace command, tracking the deleted character (left of cursor)
    /// Warning: this does not track the whole grapheme, just the character
    Backspace(Option<char>),
    /// Delete command, tracking the deleted character (right of cursor)
    /// Warning: this does not track the whole grapheme, just the character
    Delete(Option<char>),
    /// Move the cursor position
    MoveCursor,
    /// Navigated the history using up or down arrows
    HistoryNavigation,
    /// Catch-all for actions that should always form a unique undo point and never be
    /// grouped with later edits
    CreateUndoPoint,
    /// Undo/Redo actions shouldn't be reflected on the edit stack
    UndoRedo,
}

impl UndoBehavior {
    /// Return if the current operation should start a new undo set, or be
    /// combined with the previous operation
    pub fn create_undo_point_after(&self, previous: &UndoBehavior) -> bool {
        use UndoBehavior as UB;
        match (previous, self) {
            // Never start an undo set with cursor movement
            (_, UB::MoveCursor) => false,
            (UB::HistoryNavigation, UB::HistoryNavigation) => false,
            // When inserting/deleting repeatedly, each undo set should encompass
            // inserting/deleting a complete word and the associated whitespace
            (UB::InsertCharacter(c_prev), UB::InsertCharacter(c_new)) => {
                (*c_prev == '\n' || *c_prev == '\r')
                    || (!c_prev.is_whitespace() && c_new.is_whitespace())
            }
            (UB::Backspace(Some(c_prev)), UB::Backspace(Some(c_new))) => {
                (*c_new == '\n' || *c_new == '\r')
                    || (c_prev.is_whitespace() && !c_new.is_whitespace())
            }
            (UB::Backspace(_), UB::Backspace(_)) => false,
            (UB::Delete(Some(c_prev)), UB::Delete(Some(c_new))) => {
                (*c_new == '\n' || *c_new == '\r')
                    || (c_prev.is_whitespace() && !c_new.is_whitespace())
            }
            (UB::Delete(_), UB::Delete(_)) => false,
            (_, _) => true,
        }
    }
}

/// Reedline supported actions.
#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug, EnumIter)]
pub enum ReedlineEvent {
    /// No op event
    None,

    /// Complete history hint (default in full)
    HistoryHintComplete,

    /// Complete a single token/word of the history hint
    HistoryHintWordComplete,

    /// Handle EndOfLine event
    ///
    /// Expected Behavior:
    ///
    /// - On empty line breaks execution to exit with [`Signal::CtrlD`]
    /// - Secondary behavior [`EditCommand::Delete`]
    CtrlD,

    /// Handle SIGTERM key input
    ///
    /// Expected behavior:
    ///
    /// Abort entry
    /// Run [`EditCommand::Clear`]
    /// Clear the current undo
    /// Bubble up [`Signal::CtrlC`]
    CtrlC,

    /// Clears the screen and sets prompt to first line
    ClearScreen,

    /// Clears the screen and the scrollback buffer
    ///
    /// Sets the prompt back to the first line
    ClearScrollback,

    /// Handle enter event
    Enter,

    /// Handle unconditional submit event
    Submit,

    /// Submit at the end of the *complete* text, otherwise newline
    SubmitOrNewline,

    /// Esc event
    Esc,

    /// Mouse
    Mouse, // Fill in details later

    /// trigger termimal resize
    Resize(u16, u16),

    /// Run these commands in the editor
    Edit(Vec<EditCommand>),

    /// Trigger full repaint
    Repaint,

    /// Navigate to the previous historic buffer
    PreviousHistory,

    /// Move up to the previous line, if multiline, or up into the historic buffers
    Up,

    /// Move down to the next line, if multiline, or down through the historic buffers
    Down,

    /// Move right to the next column, completion entry, or complete hint
    Right,

    /// Move left to the next column, or completion entry
    Left,

    /// Navigate to the next historic buffer
    NextHistory,

    /// Search the history for a string
    SearchHistory,

    /// In vi mode multiple reedline events can be chained while parsing the
    /// command or movement characters
    Multiple(Vec<ReedlineEvent>),

    /// Test
    UntilFound(Vec<ReedlineEvent>),

    /// Trigger a menu event. It activates a menu with the event name
    Menu(String),

    /// Next element in the menu
    MenuNext,

    /// Previous element in the menu
    MenuPrevious,

    /// Moves up in the menu
    MenuUp,

    /// Moves down in the menu
    MenuDown,

    /// Moves left in the menu
    MenuLeft,

    /// Moves right in the menu
    MenuRight,

    /// Move to the next history page
    MenuPageNext,

    /// Move to the previous history page
    MenuPagePrevious,

    /// Way to bind the execution of a whole command (directly returning from [`crate::Reedline::read_line()`]) to a keybinding
    ExecuteHostCommand(String),

    /// Open text editor
    OpenEditor,
}

impl Display for ReedlineEvent {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            ReedlineEvent::None => write!(f, "None"),
            ReedlineEvent::HistoryHintComplete => write!(f, "HistoryHintComplete"),
            ReedlineEvent::HistoryHintWordComplete => write!(f, "HistoryHintWordComplete"),
            ReedlineEvent::CtrlD => write!(f, "CtrlD"),
            ReedlineEvent::CtrlC => write!(f, "CtrlC"),
            ReedlineEvent::ClearScreen => write!(f, "ClearScreen"),
            ReedlineEvent::ClearScrollback => write!(f, "ClearScrollback"),
            ReedlineEvent::Enter => write!(f, "Enter"),
            ReedlineEvent::Submit => write!(f, "Submit"),
            ReedlineEvent::SubmitOrNewline => write!(f, "SubmitOrNewline"),
            ReedlineEvent::Esc => write!(f, "Esc"),
            ReedlineEvent::Mouse => write!(f, "Mouse"),
            ReedlineEvent::Resize(_, _) => write!(f, "Resize <int> <int>"),
            ReedlineEvent::Edit(_) => write!(
                f,
                "Edit: <EditCommand> or Edit: <EditCommand> value: <string>"
            ),
            ReedlineEvent::Repaint => write!(f, "Repaint"),
            ReedlineEvent::PreviousHistory => write!(f, "PreviousHistory"),
            ReedlineEvent::Up => write!(f, "Up"),
            ReedlineEvent::Down => write!(f, "Down"),
            ReedlineEvent::Right => write!(f, "Right"),
            ReedlineEvent::Left => write!(f, "Left"),
            ReedlineEvent::NextHistory => write!(f, "NextHistory"),
            ReedlineEvent::SearchHistory => write!(f, "SearchHistory"),
            ReedlineEvent::Multiple(_) => write!(f, "Multiple[ {{ ReedLineEvents, }} ]"),
            ReedlineEvent::UntilFound(_) => write!(f, "UntilFound [ {{ ReedLineEvents, }} ]"),
            ReedlineEvent::Menu(_) => write!(f, "Menu Name: <string>"),
            ReedlineEvent::MenuNext => write!(f, "MenuNext"),
            ReedlineEvent::MenuPrevious => write!(f, "MenuPrevious"),
            ReedlineEvent::MenuUp => write!(f, "MenuUp"),
            ReedlineEvent::MenuDown => write!(f, "MenuDown"),
            ReedlineEvent::MenuLeft => write!(f, "MenuLeft"),
            ReedlineEvent::MenuRight => write!(f, "MenuRight"),
            ReedlineEvent::MenuPageNext => write!(f, "MenuPageNext"),
            ReedlineEvent::MenuPagePrevious => write!(f, "MenuPagePrevious"),
            ReedlineEvent::ExecuteHostCommand(_) => write!(f, "ExecuteHostCommand"),
            ReedlineEvent::OpenEditor => write!(f, "OpenEditor"),
        }
    }
}

pub(crate) enum EventStatus {
    Handled,
    Inapplicable,
    Exits(Signal),
}
