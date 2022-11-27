-- Add down migration script here
DROP PROCEDURE IF EXISTS sp_new_payment_loan(
   input_loan_id UUID,
   input_money_amount MONEY
); 