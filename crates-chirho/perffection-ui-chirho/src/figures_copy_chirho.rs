// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Her figures pages: the report, the paste form, the file form and the review before she saves. The words live in
//! the workspace catalog (`figures_chirho`); codes from the importer and the report pick a sentence.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceFiguresCopyChirho {
    pub page_title_chirho: String,
    pub page_description_chirho: String,
    pub heading_chirho: String,
    pub lede_chirho: String,
    pub nothing_saved_chirho: String,
    pub version_label_chirho: String,
    pub as_of_label_chirho: String,
    pub unavailable_chirho: String,
    pub report_number_chirho: String,
    pub report_heading_chirho: String,
    pub report_intro_chirho: String,
    pub by_kind_caption_chirho: String,
    pub by_month_caption_chirho: String,
    pub kind_column_chirho: String,
    pub month_column_chirho: String,
    pub money_in_column_chirho: String,
    pub costs_column_chirho: String,
    pub margin_column_chirho: String,
    pub share_column_chirho: String,
    pub per_job_column_chirho: String,
    pub jobs_column_chirho: String,
    pub total_label_chirho: String,
    pub unknown_chirho: String,
    pub flags_heading_chirho: String,
    pub flag_negative_margin_chirho: String,
    pub flag_falling_share_chirho: String,
    pub flag_costs_unknown_chirho: String,
    pub flag_missing_months_chirho: String,
    pub flag_gap_in_months_chirho: String,
    pub note_label_chirho: String,
    /// Shown when some kind has no figure for some month: totals and shares are of what she gave.
    pub incomplete_note_chirho: String,
    pub paste_number_chirho: String,
    pub paste_heading_chirho: String,
    pub paste_intro_chirho: String,
    pub csv_label_chirho: String,
    pub csv_help_chirho: String,
    pub file_number_chirho: String,
    pub file_heading_chirho: String,
    pub file_intro_chirho: String,
    pub file_label_chirho: String,
    pub file_help_chirho: String,
    pub review_file_submit_chirho: String,
    pub currency_label_chirho: String,
    pub currency_help_chirho: String,
    pub source_note_label_chirho: String,
    pub source_note_help_chirho: String,
    pub review_submit_chirho: String,
    pub review_heading_chirho: String,
    pub review_lede_chirho: String,
    pub kept_heading_chirho: String,
    /// The review's summary words, one and many: "1 kind of work · 2 months".
    pub kept_kind_one_chirho: String,
    pub kept_kinds_chirho: String,
    pub kept_month_one_chirho: String,
    pub kept_months_chirho: String,
    /// Before the name of the file her figures came from, on the review.
    pub from_file_label_chirho: String,
    pub combined_heading_chirho: String,
    pub combined_line_chirho: String,
    pub refused_heading_chirho: String,
    pub refused_line_chirho: String,
    pub nothing_kept_heading_chirho: String,
    pub save_boundary_chirho: String,
    pub save_submit_chirho: String,
    pub back_label_chirho: String,
    pub saved_notice_chirho: String,
    pub conflict_notice_chirho: String,
    pub unknown_notice_chirho: String,
    pub help_heading_chirho: String,
    pub help_body_chirho: String,
    pub refusal_empty_chirho: String,
    pub refusal_too_large_chirho: String,
    pub refusal_too_many_rows_chirho: String,
    pub refusal_wrong_header_chirho: String,
    pub refusal_no_rows_kept_chirho: String,
    pub refusal_too_many_months_chirho: String,
    pub refusal_too_many_kinds_chirho: String,
    pub refusal_currency_chirho: String,
    pub refusal_unsupported_currency_chirho: String,
    pub refusal_incomplete_shape_chirho: String,
    pub refusal_note_chirho: String,
    pub refusal_amount_out_of_range_chirho: String,
    pub refusal_unreadable_form_chirho: String,
    pub refusal_file_and_paste_chirho: String,
    pub refusal_empty_file_chirho: String,
    pub refusal_file_not_text_chirho: String,
    pub refusal_other_chirho: String,
    pub row_wrong_column_count_chirho: String,
    pub row_unclosed_quote_chirho: String,
    pub row_malformed_quote_chirho: String,
    pub row_invalid_month_chirho: String,
    pub row_invalid_kind_chirho: String,
    pub row_invalid_money_in_chirho: String,
    pub row_invalid_costs_chirho: String,
    pub row_invalid_jobs_chirho: String,
}

impl WorkspaceFiguresCopyChirho {
    /// What the review keeps, counted in words that agree with the counts: "1 kind of work · 2 months".
    pub fn kept_counts_chirho(&self, kinds_chirho: usize, months_chirho: usize) -> String {
        let kind_word_chirho = if kinds_chirho == 1 {
            &self.kept_kind_one_chirho
        } else {
            &self.kept_kinds_chirho
        };
        let month_word_chirho = if months_chirho == 1 {
            &self.kept_month_one_chirho
        } else {
            &self.kept_months_chirho
        };
        format!("{kinds_chirho} {kind_word_chirho} · {months_chirho} {month_word_chirho}")
    }

    /// Her sentence for a report flag code.
    pub fn flag_sentence_chirho(&self, code_chirho: &str) -> &str {
        match code_chirho {
            "negative-margin-chirho" => &self.flag_negative_margin_chirho,
            "falling-share-chirho" => &self.flag_falling_share_chirho,
            "costs-unknown-chirho" => &self.flag_costs_unknown_chirho,
            "missing-months-chirho" => &self.flag_missing_months_chirho,
            _ => &self.flag_gap_in_months_chirho,
        }
    }

    /// Her sentence for a whole-import refusal code.
    pub fn refusal_sentence_chirho(&self, code_chirho: &str) -> &str {
        match code_chirho {
            "empty-chirho" => &self.refusal_empty_chirho,
            "too-large-chirho" => &self.refusal_too_large_chirho,
            "too-many-rows-chirho" => &self.refusal_too_many_rows_chirho,
            "wrong-header-chirho" => &self.refusal_wrong_header_chirho,
            "no-rows-kept-chirho" => &self.refusal_no_rows_kept_chirho,
            "too-many-months-chirho" => &self.refusal_too_many_months_chirho,
            "too-many-kinds-chirho" => &self.refusal_too_many_kinds_chirho,
            "invalid-currency-chirho" => &self.refusal_currency_chirho,
            "unsupported-currency-chirho" => &self.refusal_unsupported_currency_chirho,
            "kind-without-figures-chirho"
            | "month-without-figures-chirho"
            | "costs-without-money-chirho" => &self.refusal_incomplete_shape_chirho,
            "note-too-long-chirho" => &self.refusal_note_chirho,
            "amount-out-of-range-chirho" | "jobs-out-of-range-chirho" => {
                &self.refusal_amount_out_of_range_chirho
            }
            "unreadable-form-chirho" => &self.refusal_unreadable_form_chirho,
            "file-and-paste-chirho" => &self.refusal_file_and_paste_chirho,
            "empty-file-chirho" => &self.refusal_empty_file_chirho,
            "file-not-text-chirho" => &self.refusal_file_not_text_chirho,
            _ => &self.refusal_other_chirho,
        }
    }

    /// Her sentence for one refused row's code.
    pub fn row_sentence_chirho(&self, code_chirho: &str) -> &str {
        match code_chirho {
            "wrong-column-count-chirho" => &self.row_wrong_column_count_chirho,
            "unclosed-quote-chirho" => &self.row_unclosed_quote_chirho,
            "malformed-quote-chirho" => &self.row_malformed_quote_chirho,
            "invalid-month-chirho" => &self.row_invalid_month_chirho,
            "invalid-kind-chirho" => &self.row_invalid_kind_chirho,
            "invalid-money-in-chirho" => &self.row_invalid_money_in_chirho,
            "invalid-costs-chirho" => &self.row_invalid_costs_chirho,
            _ => &self.row_invalid_jobs_chirho,
        }
    }
}
