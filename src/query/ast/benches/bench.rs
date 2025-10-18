// Copyright 2021 Datafuse Labs
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

fn main() {
    divan::main()
}

// bench                     fastest       │ slowest       │ median        │ mean          │ samples │ iters
// ╰─ dummy                                │               │               │               │         │
// ├─ deep_function_call  1.238 ms      │ 1.781 ms      │ 1.355 ms      │ 1.353 ms      │ 100     │ 100
// ├─ deep_query          285.6 µs      │ 434.6 µs      │ 306.8 µs      │ 307 µs        │ 100     │ 100
// ├─ large_query         1.739 ms      │ 1.893 ms      │ 1.795 ms      │ 1.801 ms      │ 100     │ 100
// ├─ large_statement     1.745 ms      │ 1.885 ms      │ 1.807 ms      │ 1.806 ms      │ 100     │ 100
// ╰─ wide_expr           562 µs        │ 651.9 µs      │ 588.2 µs      │ 590.5 µs      │ 100     │ 100

#[divan::bench_group(max_time = 0.5)]
mod dummy {
    use databend_common_ast::parser::parse_sql;
    use databend_common_ast::parser::tokenize_sql;
    use databend_common_ast::parser::Dialect;
    use sqlparser::dialect::PostgreSqlDialect;
    use sqlparser::parser::Parser;

    const DATAFUSION_DIALECT: PostgreSqlDialect = PostgreSqlDialect {};

    #[divan::bench]
    fn select_datafusion() {
        let case = r#"SELECT * FROM my_table WHERE 1 = 1;"#;
        let stmt = Parser::parse_sql(&PostgreSqlDialect {}, case).unwrap();
        divan::black_box(stmt);
    }

    #[divan::bench]
    fn select_databend() {
        let case = r#"SELECT * FROM my_table WHERE 1 = 1;"#;
        let tokens = tokenize_sql(case).unwrap();
        let (stmt, _) = parse_sql(&tokens, Dialect::PostgreSQL).unwrap();
        divan::black_box(stmt);
    }

    #[divan::bench]
    fn with_select_datafusion() {
        let case = "
            WITH derived AS (
                SELECT MAX(a) AS max_a,
                       COUNT(b) AS b_num,
                       user_id
                FROM MY_TABLE
                GROUP BY user_id
            )
            SELECT * FROM my_table
            LEFT JOIN derived USING (user_id)
        ";
        let stmt = Parser::parse_sql(&DATAFUSION_DIALECT, case).unwrap();
        divan::black_box(stmt);
    }

    #[divan::bench]
    fn with_select_databend() {
        let case = "
            WITH derived AS (
                SELECT MAX(a) AS max_a,
                       COUNT(b) AS b_num,
                       user_id
                FROM MY_TABLE
                GROUP BY user_id
            )
            SELECT * FROM my_table
            LEFT JOIN derived USING (user_id)
        ";
        let tokens = tokenize_sql(case).unwrap();
        let (stmt, _) = parse_sql(&tokens, Dialect::PostgreSQL).unwrap();
        divan::black_box(stmt);
    }

    #[divan::bench]
    fn large_statement_1_datafusion() {
        let case = {
            let expressions = (0..1000)
                .map(|n| format!("FN_{n}(COL_{n})"))
                .collect::<Vec<_>>()
                .join(", ");
            let tables = (0..1000)
                .map(|n| format!("TABLE_{n}"))
                .collect::<Vec<_>>()
                .join(" JOIN ");
            let where_condition = (0..1000)
                .map(|n| format!("COL_{n} = {n}"))
                .collect::<Vec<_>>()
                .join(" OR ");
            let order_condition = (0..1000)
                .map(|n| format!("COL_{n} DESC"))
                .collect::<Vec<_>>()
                .join(", ");

            format!(
                "SELECT {expressions} FROM {tables} WHERE {where_condition} ORDER BY {order_condition}"
            )
        };
        let stmt = Parser::parse_sql(&DATAFUSION_DIALECT, &case).unwrap();
        divan::black_box(stmt);
    }

    #[divan::bench]
    fn large_statement_1_databend() {
        let case = {
            let expressions = (0..1000)
                .map(|n| format!("FN_{n}(COL_{n})"))
                .collect::<Vec<_>>()
                .join(", ");
            let tables = (0..1000)
                .map(|n| format!("TABLE_{n}"))
                .collect::<Vec<_>>()
                .join(" JOIN ");
            let where_condition = (0..1000)
                .map(|n| format!("COL_{n} = {n}"))
                .collect::<Vec<_>>()
                .join(" OR ");
            let order_condition = (0..1000)
                .map(|n| format!("COL_{n} DESC"))
                .collect::<Vec<_>>()
                .join(", ");

            format!(
                "SELECT {expressions} FROM {tables} WHERE {where_condition} ORDER BY {order_condition}"
            )
        };
        let tokens = tokenize_sql(&case).unwrap();
        let (stmt, _) = parse_sql(&tokens, Dialect::PostgreSQL).unwrap();
        divan::black_box(stmt);
    }

    #[divan::bench]
    fn large_statement_2_datafusion() {
        let case = r#"explain SELECT SUM(count) FROM (SELECT ((((((((((((true)and(true)))or((('614')like('998831')))))or(false)))and((true IN (true, true, (-1014651046 NOT BETWEEN -1098711288 AND -1158262473))))))or((('780820706')=('')))) IS NOT NULL AND ((((((((((true)AND(true)))or((('614')like('998831')))))or(false)))and((true IN (true, true, (-1014651046 NOT BETWEEN -1098711288 AND -1158262473))))))OR((('780820706')=(''))))) ::INT64)as count FROM t0) as res;"#;
        let stmt = Parser::parse_sql(&DATAFUSION_DIALECT, case).unwrap();
        divan::black_box(stmt);
    }

    #[divan::bench]
    fn large_statement_2_databend() {
        let case = r#"explain SELECT SUM(count) FROM (SELECT ((((((((((((true)and(true)))or((('614')like('998831')))))or(false)))and((true IN (true, true, (-1014651046 NOT BETWEEN -1098711288 AND -1158262473))))))or((('780820706')=('')))) IS NOT NULL AND ((((((((((true)AND(true)))or((('614')like('998831')))))or(false)))and((true IN (true, true, (-1014651046 NOT BETWEEN -1098711288 AND -1158262473))))))OR((('780820706')=(''))))) ::INT64)as count FROM t0) as res;"#;
        let tokens = tokenize_sql(case).unwrap();
        let (stmt, _) = parse_sql(&tokens, Dialect::PostgreSQL).unwrap();
        divan::black_box(stmt);
    }

    #[divan::bench]
    fn large_statement_3_datafusion() {
        let case = r#"SELECT SUM(count) FROM (SELECT ((((((((((((true)and(true)))or((('614')like('998831')))))or(false)))and((true IN (true, true, (-1014651046 NOT BETWEEN -1098711288 AND -1158262473))))))or((('780820706')=('')))) IS NOT NULL AND ((((((((((true)AND(true)))or((('614')like('998831')))))or(false)))and((true IN (true, true, (-1014651046 NOT BETWEEN -1098711288 AND -1158262473))))))OR((('780820706')=(''))))) ::INT64)as count FROM t0) as res;"#;
        let stmt = Parser::parse_sql(&DATAFUSION_DIALECT, case).unwrap();
        divan::black_box(stmt);
    }

    #[divan::bench]
    fn large_statement_3_databend() {
        let case = r#"SELECT SUM(count) FROM (SELECT ((((((((((((true)and(true)))or((('614')like('998831')))))or(false)))and((true IN (true, true, (-1014651046 NOT BETWEEN -1098711288 AND -1158262473))))))or((('780820706')=('')))) IS NOT NULL AND ((((((((((true)AND(true)))or((('614')like('998831')))))or(false)))and((true IN (true, true, (-1014651046 NOT BETWEEN -1098711288 AND -1158262473))))))OR((('780820706')=(''))))) ::INT64)as count FROM t0) as res;"#;
        let tokens = tokenize_sql(case).unwrap();
        let (stmt, _) = parse_sql(&tokens, Dialect::PostgreSQL).unwrap();
        divan::black_box(stmt);
    }

    #[divan::bench]
    fn deep_query_datafusion() {
        let case = r#"SELECT * FROM numbers UNION ALL SELECT * FROM numbers UNION ALL SELECT * FROM numbers UNION ALL SELECT * FROM numbers UNION ALL SELECT * FROM numbers UNION ALL SELECT * FROM numbers UNION ALL SELECT * FROM numbers UNION ALL SELECT * FROM numbers UNION ALL SELECT * FROM numbers UNION ALL SELECT * FROM numbers UNION ALL SELECT * FROM numbers UNION ALL SELECT * FROM numbers UNION ALL SELECT * FROM numbers UNION ALL SELECT * FROM numbers UNION ALL SELECT * FROM numbers UNION ALL SELECT * FROM numbers UNION ALL SELECT * FROM numbers UNION ALL SELECT * FROM numbers UNION ALL SELECT * FROM numbers UNION ALL SELECT * FROM numbers"#;
        let stmt = Parser::parse_sql(&DATAFUSION_DIALECT, case).unwrap();
        divan::black_box(stmt);
    }

    #[divan::bench]
    fn deep_query_databend() {
        let case = r#"SELECT * FROM numbers UNION ALL SELECT * FROM numbers UNION ALL SELECT * FROM numbers UNION ALL SELECT * FROM numbers UNION ALL SELECT * FROM numbers UNION ALL SELECT * FROM numbers UNION ALL SELECT * FROM numbers UNION ALL SELECT * FROM numbers UNION ALL SELECT * FROM numbers UNION ALL SELECT * FROM numbers UNION ALL SELECT * FROM numbers UNION ALL SELECT * FROM numbers UNION ALL SELECT * FROM numbers UNION ALL SELECT * FROM numbers UNION ALL SELECT * FROM numbers UNION ALL SELECT * FROM numbers UNION ALL SELECT * FROM numbers UNION ALL SELECT * FROM numbers UNION ALL SELECT * FROM numbers UNION ALL SELECT * FROM numbers"#;
        let tokens = tokenize_sql(case).unwrap();
        let (stmt, _) = parse_sql(&tokens, Dialect::PostgreSQL).unwrap();
        divan::black_box(stmt);
    }
}
