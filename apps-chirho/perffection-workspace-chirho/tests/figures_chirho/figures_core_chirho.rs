// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Her figures: the strict CSV import, the stored document's bounds and the one report, against
//! synthetic bakery, plumbing and shop figures whose answers were worked out by hand. Known answers,
//! refusals by name, unknowns that stay unknown, and byte-identical reports for the same version.
//! The cases gpt's review raised (#26569) each have a test named for them.

use perffection_workspace_chirho::figures_flow_chirho::figures_csv_chirho::{
    import_figures_csv_chirho, parse_amount_chirho, FiguresImportErrorChirho, RowRefusalChirho,
};
use perffection_workspace_chirho::figures_flow_chirho::figures_document_chirho::{
    within_stored_bound_chirho, FiguresDocumentV1Chirho, FiguresValidationErrorChirho,
    FIGURES_MAX_KINDS_CHIRHO, FIGURES_MAX_KIND_CHARS_CHIRHO, FIGURES_MAX_MONTHS_CHIRHO,
    FIGURES_MAX_NOTE_CHARS_CHIRHO, FIGURES_MAX_STORED_BYTES_CHIRHO, FIGURES_SCHEMA_CHIRHO,
};
use perffection_workspace_chirho::figures_flow_chirho::figures_report_chirho::figures_report_chirho;

const BAKERY_CHIRHO: &str = "month,kind,money_in,costs,jobs
2026-01,Bread,1200.00,400.00,300
2026-01,Cakes,800,500,20
2026-02,Bread,1300.50,450.25,320
2026-02,Cakes,900,600,25
2026-02,Catering,500,,2
2026-03,Bread,1100,380,280
2026-03,Cakes,700,800,18
2026-03,Catering,1200,700,4
";

const PLUMBING_CHIRHO: &str = "month,kind,money_in,costs,jobs\r
2025-11,Repairs,2000,1500,10\r
2026-01,Repairs,1800,2100,9\r
2026-01,Installs,5000,3000,\r
2026-01,Callouts,300,450,3\r
";

/// Every kind has a figure for every month, so the trend may be drawn.
const SHOP_CHIRHO: &str = "month,kind,money_in,costs,jobs
2026-01,Bread,100,40,
2026-01,Cakes,100,40,
2026-02,Bread,100,40,
2026-02,Cakes,300,90,
";

fn kept_chirho(text_chirho: &str) -> FiguresDocumentV1Chirho {
    let import_chirho = import_figures_csv_chirho(text_chirho, "USD", "from my bookkeeping export");
    import_chirho
        .kept_chirho
        .expect("the synthetic figures are kept")
        .document_chirho()
        .clone()
}

fn flags_of_chirho(
    document_chirho: &FiguresDocumentV1Chirho,
) -> Vec<(&'static str, Option<String>, Vec<String>)> {
    figures_report_chirho(1, document_chirho)
        .flags_chirho
        .into_iter()
        .map(|flag_chirho| {
            (
                flag_chirho.code_chirho,
                flag_chirho.kind_chirho,
                flag_chirho.months_chirho,
            )
        })
        .collect()
}

fn months_chirho(months_chirho: &[&str]) -> Vec<String> {
    months_chirho
        .iter()
        .map(|month_chirho| month_chirho.to_string())
        .collect()
}

#[test]
fn the_bakery_report_matches_the_hand_worked_answers_chirho() {
    let report_chirho = figures_report_chirho(4, &kept_chirho(BAKERY_CHIRHO));
    assert_eq!(
        (
            report_chirho.version_chirho,
            report_chirho.first_month_chirho.as_str(),
            report_chirho.as_of_month_chirho.as_str()
        ),
        (4, "2026-01", "2026-03")
    );
    assert_eq!(
        (
            report_chirho.money_in_minor_chirho,
            report_chirho.complete_chirho
        ),
        (770_050, false)
    );
    let lines_chirho: Vec<_> = report_chirho
        .kinds_chirho
        .iter()
        .map(|line_chirho| {
            (
                line_chirho.kind_chirho.as_str(),
                line_chirho.money_in_minor_chirho,
                line_chirho.costs_minor_chirho,
                line_chirho.margin_minor_chirho,
                line_chirho.share_basis_points_chirho,
                line_chirho.jobs_chirho,
                line_chirho.money_per_job_minor_chirho,
            )
        })
        .collect();
    assert_eq!(
        lines_chirho,
        [
            (
                "Bread",
                360_050,
                Some(123_025),
                Some(237_025),
                4_676,
                Some(900),
                Some(400)
            ),
            (
                "Cakes",
                240_000,
                Some(190_000),
                Some(50_000),
                3_117,
                Some(63),
                Some(3_810)
            ),
            // February's costs are unknown, so the kind's costs and margin are unknown, not a partial sum
            (
                "Catering",
                170_000,
                None,
                None,
                2_208,
                Some(6),
                Some(28_333)
            ),
        ]
    );
    let months_chirho_view: Vec<_> = report_chirho
        .months_chirho
        .iter()
        .map(|line_chirho| {
            (
                line_chirho.month_chirho.as_str(),
                line_chirho.money_in_minor_chirho,
                line_chirho.costs_minor_chirho,
                line_chirho.margin_minor_chirho,
                line_chirho.kinds_without_figures_chirho,
            )
        })
        .collect();
    assert_eq!(
        months_chirho_view,
        [
            ("2026-01", 200_000, Some(90_000), Some(110_000), 1),
            ("2026-02", 270_050, None, None, 0),
            ("2026-03", 300_000, Some(188_000), Some(112_000), 0)
        ]
    );
    // Catering has no January figure, so the figures are incomplete and no falling share is drawn.
    assert_eq!(
        flags_of_chirho(&kept_chirho(BAKERY_CHIRHO)),
        [
            (
                "costs-unknown-chirho",
                Some("Catering".to_string()),
                months_chirho(&["2026-02"])
            ),
            (
                "missing-months-chirho",
                Some("Catering".to_string()),
                months_chirho(&["2026-01"])
            ),
        ]
    );
}

#[test]
fn the_plumbing_report_names_a_loss_a_gap_and_unknown_jobs_chirho() {
    let report_chirho = figures_report_chirho(1, &kept_chirho(PLUMBING_CHIRHO));
    assert_eq!(
        (
            report_chirho.money_in_minor_chirho,
            report_chirho.complete_chirho
        ),
        (910_000, false)
    );
    let callouts_chirho = &report_chirho.kinds_chirho[2];
    assert_eq!(
        (
            callouts_chirho.margin_minor_chirho,
            callouts_chirho.share_basis_points_chirho
        ),
        (Some(-15_000), 330)
    );
    let installs_chirho = &report_chirho.kinds_chirho[1];
    assert_eq!(
        (
            installs_chirho.jobs_chirho,
            installs_chirho.money_per_job_minor_chirho,
            installs_chirho.share_basis_points_chirho
        ),
        (None, None, 5_495)
    );
    assert_eq!(
        (
            report_chirho.kinds_chirho[0].money_per_job_minor_chirho,
            report_chirho.kinds_chirho[0].share_basis_points_chirho
        ),
        (Some(20_000), 4_176)
    );
    assert_eq!(
        flags_of_chirho(&kept_chirho(PLUMBING_CHIRHO)),
        [
            (
                "missing-months-chirho",
                Some("Installs".to_string()),
                months_chirho(&["2025-11"])
            ),
            (
                "negative-margin-chirho",
                Some("Callouts".to_string()),
                vec![]
            ),
            (
                "missing-months-chirho",
                Some("Callouts".to_string()),
                months_chirho(&["2025-11"])
            ),
            ("gap-in-months-chirho", None, months_chirho(&["2025-12"])),
        ]
    );
}

#[test]
fn complete_figures_draw_the_falling_share_trend_chirho() {
    // Bread: 100/400 = 25.00% now against 100/200 = 50.00% before; Cakes rises.
    let document_chirho = kept_chirho(SHOP_CHIRHO);
    assert!(figures_report_chirho(1, &document_chirho).complete_chirho);
    assert_eq!(
        flags_of_chirho(&document_chirho),
        [(
            "falling-share-chirho",
            Some("Bread".to_string()),
            months_chirho(&["2026-02"])
        )]
    );
}

#[test]
fn an_unknown_earlier_figure_declines_the_trend_rather_than_read_it_as_zero_chirho() {
    // gpt #26569 case 3: Cakes has no January figure; read as zero it would show Bread "falling".
    let document_chirho = kept_chirho("month,kind,money_in,costs,jobs\n2026-01,Bread,100,40,\n2026-02,Bread,100,40,\n2026-02,Cakes,100,40,\n");
    assert!(!figures_report_chirho(1, &document_chirho).complete_chirho);
    assert_eq!(
        flags_of_chirho(&document_chirho),
        [(
            "missing-months-chirho",
            Some("Cakes".to_string()),
            months_chirho(&["2026-01"])
        )]
    );
}

fn document_chirho(
    money_chirho: Vec<Vec<Option<i64>>>,
    costs_chirho: Vec<Vec<Option<i64>>>,
    jobs_chirho: Vec<Vec<Option<u32>>>,
) -> FiguresDocumentV1Chirho {
    FiguresDocumentV1Chirho {
        schema_chirho: FIGURES_SCHEMA_CHIRHO.to_string(),
        currency_chirho: "USD".to_string(),
        note_chirho: String::new(),
        months_chirho: months_chirho(&["2026-01", "2026-02"]),
        kinds_chirho: vec!["Bread".to_string(), "Catering".to_string()],
        money_in_chirho: money_chirho,
        costs_chirho,
        jobs_chirho,
    }
}

fn none_grid_chirho<T>() -> Vec<Vec<Option<T>>> {
    vec![vec![None, None], vec![None, None]]
}

#[test]
fn a_shape_with_nothing_to_report_is_refused_and_never_reported_as_zero_chirho() {
    // gpt #26569 case 1: a kind with no money figure, a month with none, costs or jobs without money
    let kind_without_chirho = document_chirho(
        vec![vec![Some(100), Some(100)], vec![None, None]],
        vec![vec![Some(40), Some(40)], vec![None, None]],
        vec![vec![Some(1), Some(1)], vec![None, None]],
    );
    assert_eq!(
        kind_without_chirho.clone().validate_chirho().err(),
        Some(FiguresValidationErrorChirho::KindWithoutFiguresChirho)
    );
    let month_without_chirho = document_chirho(
        vec![vec![Some(100), None], vec![Some(50), None]],
        none_grid_chirho(),
        none_grid_chirho(),
    );
    assert_eq!(
        month_without_chirho.clone().validate_chirho().err(),
        Some(FiguresValidationErrorChirho::MonthWithoutFiguresChirho)
    );
    let costs_without_money_chirho = document_chirho(
        vec![vec![Some(100), Some(100)], vec![Some(10), None]],
        vec![vec![None, None], vec![None, Some(5)]],
        none_grid_chirho(),
    );
    assert_eq!(
        costs_without_money_chirho.validate_chirho().err(),
        Some(FiguresValidationErrorChirho::CostsWithoutMoneyChirho)
    );
    // and the report, even if handed such a shape directly, keeps them unknown rather than 0
    let report_chirho = figures_report_chirho(1, &kind_without_chirho);
    let catering_chirho = &report_chirho.kinds_chirho[1];
    assert_eq!(
        (
            catering_chirho.costs_minor_chirho,
            catering_chirho.margin_minor_chirho,
            catering_chirho.jobs_chirho,
            catering_chirho.money_per_job_minor_chirho
        ),
        (None, None, None, None)
    );
    let month_report_chirho = figures_report_chirho(1, &month_without_chirho);
    assert_eq!(
        (
            month_report_chirho.months_chirho[1].costs_minor_chirho,
            month_report_chirho.months_chirho[1].margin_minor_chirho
        ),
        (None, None)
    );
}

#[test]
fn the_same_version_gives_byte_identical_report_json_even_through_the_stored_bytes_chirho() {
    let validated_chirho = import_figures_csv_chirho(BAKERY_CHIRHO, "USD", "")
        .kept_chirho
        .unwrap();
    let first_chirho = serde_json::to_vec(&figures_report_chirho(
        7,
        validated_chirho.document_chirho(),
    ))
    .unwrap();
    let stored_chirho: FiguresDocumentV1Chirho =
        serde_json::from_slice(validated_chirho.stored_bytes_chirho()).unwrap();
    assert_eq!(
        first_chirho,
        serde_json::to_vec(&figures_report_chirho(7, &stored_chirho)).unwrap()
    );
}

#[test]
fn rows_for_the_same_month_and_kind_are_combined_and_said_so_chirho() {
    let text_chirho = "month,kind,money_in,costs,jobs\n2026-01,Bread,100,40,10\n2026-01,bread,50.5,,5\n2026-01,Cakes,20,5,1\n2026-01,Cakes,30,6,2\n";
    let import_chirho = import_figures_csv_chirho(text_chirho, "USD", "");
    let combined_chirho: Vec<_> = import_chirho
        .combined_chirho
        .iter()
        .map(|row_chirho| {
            (
                row_chirho.kind_chirho.as_str(),
                row_chirho.lines_chirho.clone(),
            )
        })
        .collect();
    assert_eq!(
        combined_chirho,
        [("Bread", vec![2, 3]), ("Cakes", vec![4, 5])]
    );
    let document_chirho = import_chirho.kept_chirho.unwrap().document_chirho().clone();
    // first-seen spelling; money summed; Bread's second row knew no costs, so its costs are unknown
    assert_eq!(document_chirho.kinds_chirho, ["Bread", "Cakes"]);
    assert_eq!(
        document_chirho.money_in_chirho,
        [vec![Some(15_050)], vec![Some(5_000)]]
    );
    assert_eq!(
        document_chirho.costs_chirho,
        [vec![None], vec![Some(1_100)]]
    );
    assert_eq!(document_chirho.jobs_chirho, [vec![Some(15)], vec![Some(3)]]);
}

#[test]
fn a_row_that_cannot_be_read_is_kept_out_with_its_line_and_reason_chirho() {
    let text_chirho = "month,kind,money_in,costs,jobs
2026-13,Bread,100,,
2026-01,Bread,$100,,
2026-01,Bread,1,200.00,,
2026-01,Bread,-5,,
2026-01,Bread,12.345,,
2026-01,Bread,10,,x
2026-01,,10,,
2026-01,\"Weddings, \"\"large\"\"\",10,,
2026-01,\"open quote,10,,
2026-01,Bread,10,4.,
2026-01,Bread,\"1\"0,4,1
2026-01,Br\"ead,10,,
";
    let import_chirho = import_figures_csv_chirho(text_chirho, "USD", "");
    let refused_chirho: Vec<_> = import_chirho
        .refused_chirho
        .iter()
        .map(|row_chirho| (row_chirho.line_chirho, row_chirho.refusal_chirho))
        .collect();
    assert_eq!(
        refused_chirho,
        [
            (2, RowRefusalChirho::InvalidMonthChirho),
            (3, RowRefusalChirho::InvalidMoneyInChirho),
            (4, RowRefusalChirho::WrongColumnCountChirho),
            (5, RowRefusalChirho::InvalidMoneyInChirho),
            (6, RowRefusalChirho::InvalidMoneyInChirho),
            (7, RowRefusalChirho::InvalidJobsChirho),
            (8, RowRefusalChirho::InvalidKindChirho),
            (10, RowRefusalChirho::UnclosedQuoteChirho),
            (11, RowRefusalChirho::InvalidCostsChirho),
            // gpt #26569 case 2: never read as 10
            (12, RowRefusalChirho::MalformedQuoteChirho),
            (13, RowRefusalChirho::MalformedQuoteChirho),
        ]
    );
    // the one good row, a quoted label holding a comma and a doubled quote, is kept exactly
    assert_eq!(import_chirho.kept_rows_chirho, 1);
    assert_eq!(
        import_chirho
            .kept_chirho
            .unwrap()
            .document_chirho()
            .kinds_chirho,
        ["Weddings, \"large\""]
    );
}

#[test]
fn past_a_bound_the_whole_import_is_refused_by_name_never_truncated_chirho() {
    let header_chirho = "month,kind,money_in,costs,jobs\n";
    let refusal_chirho = |text_chirho: &str| {
        import_figures_csv_chirho(text_chirho, "USD", "")
            .kept_chirho
            .err()
    };
    let months_text_chirho: String = (0..=FIGURES_MAX_MONTHS_CHIRHO)
        .map(|index_chirho| {
            format!(
                "{}-{:02},Bread,1,,\n",
                2020 + index_chirho / 12,
                index_chirho % 12 + 1
            )
        })
        .collect();
    assert_eq!(
        refusal_chirho(&format!("{header_chirho}{months_text_chirho}")),
        Some(FiguresImportErrorChirho::InvalidChirho(
            FiguresValidationErrorChirho::TooManyMonthsChirho
        ))
    );
    let kinds_chirho: String = (0..=FIGURES_MAX_KINDS_CHIRHO)
        .map(|index_chirho| format!("2026-01,Kind {index_chirho},1,,\n"))
        .collect();
    assert_eq!(
        refusal_chirho(&format!("{header_chirho}{kinds_chirho}")),
        Some(FiguresImportErrorChirho::InvalidChirho(
            FiguresValidationErrorChirho::TooManyKindsChirho
        ))
    );
    assert_eq!(
        refusal_chirho(&format!(
            "{header_chirho}{}",
            "2026-01,Bread,1,,\n".repeat(1_001)
        )),
        Some(FiguresImportErrorChirho::TooManyRowsChirho)
    );
    assert_eq!(
        refusal_chirho(&format!("{header_chirho}{}", " ".repeat(70_000))),
        Some(FiguresImportErrorChirho::TooLargeChirho)
    );
    assert_eq!(
        refusal_chirho("month,kind,amount\n2026-01,Bread,1\n"),
        Some(FiguresImportErrorChirho::WrongHeaderChirho)
    );
    assert_eq!(
        refusal_chirho("\n  \n"),
        Some(FiguresImportErrorChirho::EmptyChirho)
    );
    assert_eq!(
        refusal_chirho(&format!("{header_chirho}2026-99,Bread,1,,\n")),
        Some(FiguresImportErrorChirho::NoRowsKeptChirho)
    );
}

#[test]
fn only_verified_two_decimal_currencies_are_supported_chirho() {
    let currency_chirho = |code_chirho: &str| {
        import_figures_csv_chirho(
            "month,kind,money_in,costs,jobs\n2026-01,Bread,1,,\n",
            code_chirho,
            "",
        )
        .kept_chirho
        .err()
    };
    for supported_chirho in ["USD", "EUR", "GBP", "CAD"] {
        assert_eq!(
            currency_chirho(supported_chirho),
            None,
            "{supported_chirho}"
        );
    }
    for unsupported_chirho in ["JPY", "KWD", "ABC", "XAU"] {
        assert_eq!(
            currency_chirho(unsupported_chirho),
            Some(FiguresImportErrorChirho::InvalidChirho(
                FiguresValidationErrorChirho::UnsupportedCurrencyChirho
            )),
            "{unsupported_chirho}"
        );
    }
    for malformed_chirho in ["usd", "US", "USDX", ""] {
        assert_eq!(
            currency_chirho(malformed_chirho),
            Some(FiguresImportErrorChirho::InvalidChirho(
                FiguresValidationErrorChirho::InvalidCurrencyChirho
            )),
            "{malformed_chirho}"
        );
    }
}

#[test]
fn the_largest_saveable_figures_are_measured_and_the_exact_byte_bound_holds_chirho() {
    // The maximum under these bounds: every label and the note filled with distinct 4-byte characters
    // (JSON keeps them as 4 bytes; quote and backslash escape to 2), every amount and job at its cap.
    let wide_chirho = |seed_chirho: u32, count_chirho: usize| {
        (0..count_chirho)
            .map(|offset_chirho| {
                char::from_u32(0x1F300 + seed_chirho * 64 + offset_chirho as u32).unwrap()
            })
            .collect::<String>()
    };
    let document_chirho = FiguresDocumentV1Chirho {
        schema_chirho: FIGURES_SCHEMA_CHIRHO.to_string(),
        currency_chirho: "USD".to_string(),
        note_chirho: wide_chirho(20, FIGURES_MAX_NOTE_CHARS_CHIRHO),
        months_chirho: (0..FIGURES_MAX_MONTHS_CHIRHO)
            .map(|index_chirho| {
                format!("{}-{:02}", 2024 + index_chirho / 12, index_chirho % 12 + 1)
            })
            .collect(),
        kinds_chirho: (0..FIGURES_MAX_KINDS_CHIRHO as u32)
            .map(|index_chirho| wide_chirho(index_chirho, FIGURES_MAX_KIND_CHARS_CHIRHO))
            .collect(),
        money_in_chirho: vec![
            vec![Some(99_999_999_999); FIGURES_MAX_MONTHS_CHIRHO];
            FIGURES_MAX_KINDS_CHIRHO
        ],
        costs_chirho: vec![
            vec![Some(99_999_999_999); FIGURES_MAX_MONTHS_CHIRHO];
            FIGURES_MAX_KINDS_CHIRHO
        ],
        jobs_chirho: vec![vec![Some(999_999); FIGURES_MAX_MONTHS_CHIRHO]; FIGURES_MAX_KINDS_CHIRHO],
    };
    let stored_chirho = document_chirho
        .validate_chirho()
        .expect("the largest is saveable")
        .stored_bytes_chirho()
        .len();
    assert!(
        stored_chirho < FIGURES_MAX_STORED_BYTES_CHIRHO,
        "{stored_chirho} bytes"
    );
    println!(
        "largest saveable figures: {stored_chirho} of {FIGURES_MAX_STORED_BYTES_CHIRHO} bytes"
    );
    // the exact serialized-byte bound itself: the limit is stored, one byte past it is refused
    assert_eq!(
        within_stored_bound_chirho(
            &vec![b'x'; FIGURES_MAX_STORED_BYTES_CHIRHO],
            FIGURES_MAX_STORED_BYTES_CHIRHO
        ),
        Ok(())
    );
    assert_eq!(
        within_stored_bound_chirho(
            &vec![b'x'; FIGURES_MAX_STORED_BYTES_CHIRHO + 1],
            FIGURES_MAX_STORED_BYTES_CHIRHO
        ),
        Err(FiguresValidationErrorChirho::TooLargeToStoreChirho)
    );
}

#[test]
fn amounts_are_plain_decimals_with_at_most_two_places_chirho() {
    let parsed_chirho: Vec<_> = [
        "0",
        "7",
        "7.5",
        "7.05",
        "999999999.99",
        "1234567890",
        "7.",
        ".5",
        "1e3",
        "+7",
        " 7",
        "7,00",
    ]
    .iter()
    .map(|text_chirho| parse_amount_chirho(text_chirho))
    .collect();
    assert_eq!(
        parsed_chirho,
        [
            Some(0),
            Some(700),
            Some(750),
            Some(705),
            Some(99_999_999_999),
            None,
            None,
            None,
            None,
            None,
            None,
            None
        ]
    );
}
