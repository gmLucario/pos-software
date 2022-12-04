-- Add down migration script here
DROP FUNCTION IF EXISTS sp_earnings(
	start_date VARCHAR(10),
	end_date VARCHAR(10)
);
