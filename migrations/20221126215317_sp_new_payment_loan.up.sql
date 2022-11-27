-- Add up migration script here
CREATE OR REPLACE PROCEDURE sp_new_payment_loan(
   input_loan_id UUID,
   input_money_amount MONEY
)
language plpgsql    
AS $$
BEGIN
DECLARE
    total_payments loan_payment.money_amount%TYPE;
    total_loan loan.price%TYPE;
    new_status_loan loan.status_loan%TYPE;

    BEGIN

        INSERT INTO loan_payment (loan_id, money_amount) VALUES (input_loan_id, input_money_amount);

        SELECT
            SUM(lp.money_amount) into total_payments
        FROM loan_payment lp 
        WHERE lp.loan_id = input_loan_id;

        SELECT
            loan.price into total_loan
        FROM loan
        WHERE loan.id =  input_loan_id;

        SELECT
            CASE 
                WHEN total_payments >= total_loan THEN 1
                ELSE 2
            END into new_status_loan;

        UPDATE loan 
        SET
            status_loan = new_status_loan
        WHERE id =  input_loan_id;        

    END;

    COMMIT;
END;$$
