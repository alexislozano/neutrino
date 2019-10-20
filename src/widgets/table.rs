use crate::utils::event::Event;
use crate::utils::style::{inline_style, scss_to_css};
use crate::widgets::widget::Widget;
use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};

/// # The state of a Table
///
/// ## Fields
///
/// ```text
/// headers: Option<Vec<String>>
/// rows: Vec<Vec<String>>
/// stretched: bool
/// style: String
/// ```
pub struct TableState {
    headers: Option<Vec<String>>,
    rows: Vec<Vec<String>>,
    stretched: bool,
    style: String,
}

impl TableState {
    /// Get the headers
    pub fn headers(&self) -> Option<&Vec<String>> {
        self.headers.as_ref()
    }

    /// Get the rows
    pub fn rows(&self) -> &Vec<Vec<String>> {
        &self.rows
    }

    /// Get the stretched flag
    pub fn stretched(&self) -> bool {
        self.stretched
    }

    /// Get the style
    pub fn style(&self) -> &str {
        &self.style
    }

    fn set_headers_impl(&mut self, headers: Vec<&str>, cleanslate: bool) {
        if cleanslate && !self.rows.is_empty() {
            self.rows.clear();
        }

        self.headers = Some(headers
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<String>>()
        );
    }

    /// Set the headers
    ///
    /// Note that in case rows are not empty, calling this method will remove all existing rows,
    /// due to the potential discrepancy in column counts between headers and rows.
    /// To suppress this behavior, use [`set_headers_unchecked`] method instead.
    ///
    /// [`set_headers_unchecked`]: #method.set_headers_unchecked
    pub fn set_headers(&mut self, headers: Vec<&str>) {
        self.set_headers_impl(headers, true);
    }

    /// Set the headers, without removing all the existing rows
    ///
    /// This is for when you can guarantee that the column counts between headers and rows will
    /// match.
    pub fn set_headers_unchecked(&mut self, headers: Vec<&str>) {
        self.set_headers_impl(headers, false);
    }

    /// Set rows with new content
    ///
    /// Note that this will effectively replace the entire table content, if any, with given ones.
    ///
    /// # Errors
    /// A variant of [`TableError`] will be returned in case the column count check fails due to:
    /// * the `rows` vector containing some rows with uneven column counts, or
    /// * the column count of the header and `rows` being not equal.
    ///
    /// [`TableError`]: enum.TableError.html
    pub fn set_rows(&mut self, rows: Vec<Vec<&str>>) -> Result<(), TableError> {
        let mut within_rows = rows.iter().map(|r| r.len()).collect::<Vec<_>>();
        within_rows.sort_unstable();
        within_rows.dedup();
        match within_rows.len() {
            0 => {}
            1 => {
                if let Some(headers) = self.headers.as_ref() {
                    if headers.len() != within_rows.pop().unwrap() {
                        return Err(TableError::ColumnCountMismatchHeaderRow);
                    }
                }
            }
            _ => return Err(TableError::ColumnCountMismatchWithinRows),
        }

        self.rows = rows
            .iter()
            .map(|r| r.iter().map(|c| c.to_string()).collect::<Vec<_>>())
            .collect::<Vec<_>>();

        Ok(())
    }

    /// Add a row
    ///
    /// # Errors
    /// A variant of [`TableError`] will be returned in case the column count check fails due to:
    /// * the column count of the header and the `row` being not equal, or
    /// * the table does not have a header but the column count of existing rows and this `row`
    ///   being not equal.
    ///
    /// [`TableError`]: enum.TableError.html
    pub fn add_row(&mut self, row: Vec<&str>) -> Result<(), TableError> {
        if let Some(headers) = self.headers.as_ref() {
            if headers.len() != row.len() {
                return Err(TableError::ColumnCountMismatchHeaderRow);
            }
        } else if !self.rows.is_empty()
            && self.rows.first().as_ref().unwrap().len() != row.len()
        {
            return Err(TableError::ColumnCountMismatchWithinRows);
        }

        self.rows.push(row
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<String>>()
        );

        Ok(())
    }

    /// Remove a row
    pub fn remove_row(&mut self, index: usize) -> Vec<String> {
        self.rows.remove(index)
    }

    /// Remove all rows
    pub fn remove_all_rows(&mut self) {
        self.rows.clear();
    }

    /// Set the stretched flag
    pub fn set_stretched(&mut self, stretched: bool) {
        self.stretched = stretched;
    }

    /// Set the style
    pub fn set_style(&mut self, style: &str) {
        self.style = style.to_string();
    }
}

/// Error type denoting possible failures of Table manipulation
#[derive(Debug)]
pub enum TableError {
    /// For when the column counts between the header and rows do not match
    ColumnCountMismatchHeaderRow,
    /// For when the column counts within rows do not match
    ColumnCountMismatchWithinRows,
}
impl Display for TableError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(
            f,
            "{}",
            match self {
                TableError::ColumnCountMismatchHeaderRow => {
                    "column counts between header and row(s) do not match"
                }
                TableError::ColumnCountMismatchWithinRows => {
                    "column counts within non-header rows do not match"
                }
            }
        )
    }
}
impl Error for TableError {}

/// # The listener of a Table
pub trait TableListener {
    /// Function triggered on update event
    fn on_update(&self, state: &mut TableState);
}

pub struct Table {
    name: String,
    state: TableState,
    listener: Option<Box<dyn TableListener>>,
}

impl Table {
    /// Create a Table
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            state: TableState {
                headers: None,
                rows: vec![],
                stretched: false,
                style: "".to_string(),
            },
            listener: None,
        }
    }

    /// Set the headers
    ///
    /// Note that in case rows are not empty, calling this method will remove all existing rows,
    /// due to the potential discrepancy in column counts between headers and rows.
    /// To suppress this behavior, use [`set_headers_unchecked`] method instead.
    ///
    /// [`set_headers_unchecked`]: #method.set_headers_unchecked
    pub fn set_headers(&mut self, headers: Vec<&str>) {
        self.state.set_headers(headers);
    }

    /// Set the headers, without removing all the existing rows
    ///
    /// This is for when you can guarantee that the column counts between headers and rows will
    /// match.
    pub fn set_headers_unchecked(&mut self, headers: Vec<&str>) {
        self.state.set_headers_unchecked(headers);
    }

    /// Add a row
    ///
    /// # Errors
    /// A variant of [`TableError`] will be returned in case the column count check fails due to:
    /// * the column count of the header and the `row` being not equal, or
    /// * the table does not have a header but the column count of existing rows and this `row`
    ///   being not equal.
    ///
    /// [`TableError`]: enum.TableError.html
    pub fn add_row(&mut self, row: Vec<&str>) -> Result<(), TableError> {
        self.state.add_row(row)
    }

    /// Set the stretched flag to true
    pub fn set_stretched(&mut self) {
        self.state.set_stretched(true);
    }

    /// Set the listener
    pub fn set_listener(&mut self, listener: Box<dyn TableListener>) {
        self.listener = Some(listener);
    }

    /// Set the style
    pub fn set_style(&mut self, style: &str) {
        self.state.set_style(style);
    }
}

// <table>
//     <thead>
//         <tr>
//             <th colspan="2">The table header</th>
//         </tr>
//     </thead>
//     <tbody>
//         <tr>
//             <td>The table body</td>
//             <td>with two columns</td>
//         </tr>
//     </tbody>
// </table>

impl Widget for Table {
    fn eval(&self) -> String {
        let stretched = if self.state.stretched() {
            "stretched"
        } else {
            ""
        };
        let style = inline_style(&scss_to_css(&format!(
            r##"#{}{{{}}}"##,
            self.name,
            self.state.style(),
        )));
        let mut html = format!(
            r#"<div id="{}" class="table {}"><table>"#,
            self.name,
            stretched,
        );
        match self.state.headers() {
            None => (),
            Some(headers) => {
                html.push_str("<tr>");
                for (index, header) in headers.iter().enumerate() {
                    let last = if index == headers.len() - 1 {
                        "last"
                    } else {
                        ""
                    };
                    html.push_str(&format!(
                        r#"<th class="{}">{}</th>"#,
                        last,
                        header,
                    ));
                }
                html.push_str("</tr>");
            }
        };
        for (index, row) in self.state.rows().iter().enumerate() {
            let parity = if index % 2 == 0 {
                "even"
            } else {
                "odd"
            };
            html.push_str("<tr>");
            for item in row.iter() {
                html.push_str(&format!(
                    r#"<td class="{}">{}</td>"#,
                    parity,
                    item
                ));
            }
            html.push_str("</tr>");
        }
        html.push_str("</table></div>");
        format!("{}{}", style, html)
    }

    fn trigger(&mut self, event: &Event) {
        match event {
            Event::Update => self.on_update(),
            Event::Change { source, value } => {
                if source == &self.name {
                    self.on_change(value)
                }
            }
            _ => (),
        }
    }

    fn on_update(&mut self) {
        match &self.listener {
            None => (),
            Some(listener) => {
                listener.on_update(&mut self.state);
            }
        }
    }

    fn on_change(&mut self, _value: &str) {}
}
