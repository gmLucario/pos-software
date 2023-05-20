-- Add up migration script here
CREATE OR REPLACE FUNCTION sp_earnings(
	start_date VARCHAR(10),
	end_date VARCHAR(10)
)
RETURNS MONEY as $$
	DECLARE
	    money_loans operation.earning%TYPE;
	    money_invested_catalog operation.earning%TYPE;

	BEGIN
		SELECT
			COALESCE(SUM(loan.price), 0::MONEY) INTO money_loans
		FROM loan
		LEFT JOIN sale ON (
			sale.id = loan.id
		)
		WHERE
			loan.status_loan != 1
			AND to_date(sale.sold_at::text, 'YYYY/MM/DD')
			BETWEEN
				to_date(start_date, 'YYYY-MM-DD')
				AND to_date(end_date, 'YYYY-MM-DD');

	    SELECT
	        COALESCE(SUM(c."cost" * c.current_amount), 0::MONEY) INTO money_invested_catalog
	    FROM "catalog" c
		WHERE
			to_date(c.priced_at::text, 'YYYY/MM/DD')
			BETWEEN
				to_date(start_date, 'YYYY-MM-DD')
				AND to_date(end_date, 'YYYY-MM-DD');

        RETURN (
		    SELECT
		        COALESCE(SUM(o.earning), 0::MONEY) - (money_loans + money_invested_catalog)
		    FROM operation o
			WHERE
				to_date(o.recorded_at::text, 'YYYY/MM/DD')
				BETWEEN
					to_date(start_date, 'YYYY-MM-DD')
					AND to_date(end_date, 'YYYY-MM-DD')
				AND o.condition_id = 1
        );
	END;
$$
LANGUAGE plpgsql;
