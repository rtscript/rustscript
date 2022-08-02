use std::collections::HashMap;

use crate::context::*;
use crate::error::*;
use crate::ast::*;
use crate::token::*;
use crate::token_type::*;
use crate::object::*;



#[derive(Debug)]
pub struct Parser<'a> {
    tokens: &'a [Token],
    current: usize,
    had_error: bool,
    ast_tokens: Vec<AstToken>,
    context: Context,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &[Token]) -> Parser {
        Parser {
            tokens,
            current: 0,
            had_error: false,
            ast_tokens: Vec::new(),
            context: Context::new(),
        }
    }

    pub fn parse(&mut self) -> Result<&Vec<AstToken>, Problem> { 
        while !self.is_at_end() {
            self.declaration(); 
        }
        if self.had_error {
            Err(Problem::fail())
        } else {
            Ok(&self.ast_tokens)
        }
    }
   
    fn consume(&mut self, ttype: TokenType, message: &str) -> Result<Token, Problem> {
        if self.check(ttype) {
            Ok(self.advance().dup())
        } else {
            let peek = self.peek().dup();
            Err(self.error(&peek, message))
        }
    }

    fn error(&mut self, token: &Token, message: &str) -> Problem {
        self.had_error = true;
        Problem::parse_error(token, message)
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().is(TokenType::SemiColon) {
                return;
            }

            if matches!(
                self.peek().token_type(),
                TokenType::Class
                    | TokenType::Fn
                    | TokenType::Let
                    | TokenType::For
                    | TokenType::If
                    | TokenType::While
                    | TokenType::Print
                    | TokenType::Return
            ) {
                return;
            }

            self.advance();
        }
    }

    fn is_match(&mut self, types: &[TokenType]) -> bool {
        for &t in types {
            if self.check(t) {
                self.advance();
                return true;
            }
        }

        false
    }

    fn check(&self, ttype: TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            self.peek().is(ttype)
        }
    }

    fn check_further(&self, ttype: TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            self.peek_further().is(ttype)
        }
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().is(TokenType::Eof)
    }

    fn peek_further(&self) -> &Token {
        self.tokens.get(self.current + 1).unwrap()
    }

    fn peek(&self) -> &Token {
        self.tokens.get(self.current).unwrap()
    }

    fn previous(&self) -> &Token {
        self.tokens.get(self.current - 1).unwrap()
    }

    fn add_token(&mut self, atype: AstType) {
        self.add_token_object(atype, None);
    }
    fn add_token_with_value(&mut self, atype: AstType, lexeme: String) {
        self.ast_tokens.push(AstToken::new(atype, lexeme, None));
    }

    fn add_token_object(&mut self, atype: AstType, literal: Option<Object>) {
        let lexeme = self.previous().as_string();
        self.ast_tokens.push(AstToken::new(atype, lexeme, literal));
    }

    fn declaration(&mut self) -> Result<(), Problem> {
        //go to 3 different functions + statement based on is_match
        let result = if self.is_match(&[TokenType::Class]) {
            self.class_declaration()
        } else if self.is_match(&[TokenType::Fn]) {
            self.function("function")
        } else if self.is_match(&[TokenType::Let]) {
            self.var_declaration()
        } else if self.check(TokenType::For) || 
            self.check(TokenType::If) || 
            self.check(TokenType::Print) || 
            self.check(TokenType::Return) ||
            self.check(TokenType::While) || 
            self.check(TokenType::Break) {
            self.statement()
        } else if self.check(TokenType::Identifier) && self.check_further(TokenType::LeftParen) {
            self.function_call()
        } else {
            Err(Problem::fail())
        };
        
        if result.is_err() {
            self.synchronize();
        }
    
        result
    }

   

    fn type_store(&mut self) ->  Result<(), Problem> {
        self.consume(TokenType::Identifier, "Variable name Required")?;
        self.add_token(AstType::Identifier);
        self.consume(TokenType::Annotation, "Variable must be annotated with : [variable name: variable type] ")?;
        
        let var_type = self.peek().token_type();
        if matches!(var_type, TokenType::NumberType | TokenType::StringType | TokenType::BoolType) {
            self.consume(var_type, "must have type name")?;
        }

        Ok(())
    }

    fn function(&mut self, kind: &str) -> Result<(), Problem> {

        let mut local_context = HashMap::new();
        local_context.insert("name", 90);

        self.add_token(AstType::Fn);

        self.consume(TokenType::Identifier, &format!("Jparser: Expect {kind} name"))?;
        self.add_token(AstType::Identifier);
    
        self.consume(TokenType::LeftParen, &format!("Jparser: Expect '(' after {kind} name."))?;
        self.add_token(AstType::LeftParen);
    


        if !self.check(TokenType::RightParen) {
            self.type_store()?;

            while self.is_match(&[TokenType::Comma]) {
                self.add_token(AstType::Comma);
                self.type_store()?;
            }
        }
        
        //Close function paramaters with )
        self.consume(TokenType::RightParen, "Jparser: Expect ')' after parameters.")?;
        self.add_token(AstType::RightParen);

        //Returns go here

        self.context.establish_context("add".to_string(), "num".to_string(), "num".to_string())?;
    
        //left brace {
        self.consume(TokenType::LeftBrace, &format!("Jparser: Expect '{{' before {kind} body."))?;
    
        // check for block, ie end paramater
        self.block()?;

        Ok(())
    }

    fn function_call(&mut self) -> Result<(), Problem>  {
        
        //Function name 
        self.consume(TokenType::Identifier, "Function call name not added")?;
        self.add_token(AstType::Identifier);
        
        //Left Paranthesis (
        self.consume(TokenType::LeftParen, "Function Opening brace not added")?;
        self.add_token(AstType::LeftParen);


        if !self.check(TokenType::RightParen) {
            
            self.consume(TokenType::Identifier, "Expect paramater name")?;
            self.add_token(AstType::Identifier);

            while self.is_match(&[TokenType::Comma]) {
                self.consume(TokenType::Identifier, "Jparser: Expect paramter name")?;
                self.add_token(AstType::Identifier);
            }
        }

        //function call end parenthesis )
        self.consume(TokenType::RightParen, "Close function call body")?;
        self.add_token(AstType::RightParen);

        //Check for return context if not there
        self.consume(TokenType::SemiColon, "End function call with semi-colon")?;
        self.add_token(AstType::SemiColon);

        Ok(())
    }

    fn var_values(&mut self) -> Result<(), Problem> {
        if self.is_match(&[TokenType::String]) {
            self.add_token(AstType::String);
        } else if self.is_match(&[TokenType::Number]) {
            self.add_token(AstType::Number);
        } else {
            println!("Unexpect variable name");
        }
        Ok(())
    }
    
    //self.expression()
    fn var_declaration(&mut self) -> Result<(), Problem> {
        
        //let
        self.add_token(AstType::Let);
    
        //variable name
        let name = self.consume(TokenType::Identifier, "Jparser: Expect variable name.")?;

        // println!("{:?}", &name);

        self.add_token_object(AstType::Identifier, name.literal);
    


        self.consume(TokenType::Assign, "Jparser: Expect '=' variable assignment required.")?;
        self.add_token(AstType::Assign);

        self.var_values()?;
    
        self.consume(TokenType::SemiColon, "Jparser: Expect ';' after variable declaration.")?;
        self.add_token(AstType::SemiColon);
         
        Ok(())
    }

    fn print_statement(&mut self) -> Result<(), Problem> {
        // console.log
        self.add_token(AstType::Print);
        
        // (
        self.consume(TokenType::LeftParen, "Print Requires Opening brace")?;
        self.add_token(AstType::LeftParen);

        // ""
        self.consume(TokenType::String, "No string Value for  print")?;
        

        let mut string_value = String::from("");

        if let Some(print_val) = &self.previous().literal {
            string_value = print_val.to_string();
        }

        let mut params = Vec::new();

        //if right paren not deteced. 
        if !self.check(TokenType::RightParen) {
            //While there's a comma, consume comma, is_match does that automatically
            while self.is_match(&[TokenType::Comma]) {
                let token_next = self.peek().token_type();
                params.push(format!("${{{}}}", self.peek().as_string())); // if not closing brace ie "{}, )"
                self.consume(token_next, "Println: Expect paramter value after comma")?;
            }
        }

        for (index, p) in params.iter().enumerate() {
            string_value = string_value.replacen("{}", &p, 1);
        }

        string_value = format!("`{}`", string_value);

   

        self.add_token_with_value(AstType::String, string_value);

        self.consume(TokenType::RightParen, "Print Requires Closing brace")?;
        self.add_token(AstType::RightParen);

        self.consume(TokenType::SemiColon, "Print value must end with ;")?;
        self.add_token(AstType::SemiColon);
        Ok(())
    }
    
    fn block(&mut self) -> Result<(), Problem> {
        self.add_token(AstType::LeftBrace);
    
        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            self.declaration()?;
        }
    
        self.consume(TokenType::RightBrace, "Jparser: Expect '}' after block.")?;
        self.add_token(AstType::RightBrace);
    
        Ok(())
    }
    
    
    fn statement(&mut self) -> Result<(), Problem> {
    
        //if user right breaks, duplicate current token, Jparser: expect ; and return
        if self.is_match(&[TokenType::Break]) {
            let token = self.previous().dup();
            self.consume(TokenType::SemiColon, "Jparser: Expect ';' after break statement.")?;
            return Ok(())
        }
    
        //if for then for_statement
        if self.is_match(&[TokenType::For]) {
            return self.for_statement();
        }
    
        //if if then return if statement
        if self.is_match(&[TokenType::If]) {
            return Ok(self.if_statement()?);
        }
    
        if self.is_match(&[TokenType::Print]) {
            return Ok(self.print_statement()?);
        }
    
        if self.is_match(&[TokenType::Return]) {
            return Ok(self.return_statement()?);
        }
    
        if self.is_match(&[TokenType::While]) {
            return Ok(self.while_statement()?);
        }
    
        //closure block
        if self.is_match(&[TokenType::LeftBrace]) {
            return Ok(self.block()?);
        }

        Ok(())
    
        // self.expression_statement()
    }

   

    // for (var i = 0; i < 10; i = i + 1) print i;
    fn for_statement(&mut self) -> Result<(), Problem> {
    
        //Opening (
        self.consume(TokenType::LeftParen, "Jparser: Expect '(' after 'for'.")?;
    
        // ; = return.   var goes to var_declaration, else expression statement
        let initializer = if self.is_match(&[TokenType::SemiColon]) {
            None
        } else if self.is_match(&[TokenType::Let]) {
            Some(self.var_declaration()?)
        } else {
            Some(self.expression_statement()?)
        };
    
        //if semicolon return, else another expression
        let condition = if self.check(TokenType::SemiColon) {
            None
        } else {
            Some(self.expression()?)
        };
    
        //last semi-colon ends for loop params
        self.consume(TokenType::SemiColon, "Jparser: Expect ';' after loop condition.")?;
    
        //now expefct a right parenthesis, if not check for another expression
        let increment = if self.check(TokenType::RightParen) {
            None
        } else {
            Some(self.expression()?)
        };
    
        //now check for a right parenthesis
        self.consume(TokenType::RightParen, "Jparser: Expect ')' after for clauses.")?;
    
        //check for statements, maybe block statement goes here
        // let mut body = self.statement()?;
    
        Ok(())
    }
    
    fn if_statement(&mut self) -> Result<(), Problem> {
    
        //left param
        self.consume(TokenType::LeftParen, "Jparser: Expect '(' after 'if'.")?;
        //expression
        self.expression()?;
        //righ param
        self.consume(TokenType::RightParen, "Jparser: Expect ')' after 'if'.")?;
    
        //get the branch {}
        self.statement()?;
        //match else then statement again
        if self.is_match(&[TokenType::Else]) {
            self.statement()?
        };
    
        Ok(())
    }
    
    
    
    fn return_statement(&mut self) -> Result<(), Problem> {
        let keyword = self.previous().dup();
        
        if !self.check(TokenType::SemiColon) {
            self.expression()?
        };
    
        self.consume(TokenType::SemiColon, "Jparser: Expect ';' after return value.")?;
    
        Ok(())
    }
    
    
    fn while_statement(&mut self) -> Result<(), Problem> {
        self.consume(TokenType::LeftParen, "Jparser: Expect '(' after 'while'.")?;
        self.expression()?;
        self.consume(TokenType::RightParen, "Jparser: Expect ')' after 'while'.")?;
        self.statement()?;
    
        Ok(())
    }
    
    fn class_declaration(&mut self) -> Result<(), Problem> {
    
        // if no class name then return error
        let name = self.consume(TokenType::Identifier, "Jparser: Expect class name.")?;
    
        //if there's a < symbol then check for identifier for superclass name    
        if self.is_match(&[TokenType::Less]) {
            self.consume(TokenType::Identifier, "Jparser: Expect superclass name.")?;
        } 
    
        // Should be an { body next
        self.consume(TokenType::LeftBrace, "Jparser: Expect '{' before class body.")?;
    
        //while right brace is not mentioned yet, push self.function into methods vec
        let mut methods = Vec::new();
        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            methods.push(self.function("method")?);
        }
    
        // errror check for }
        self.consume(TokenType::RightBrace, "Jparser: Expect '}' after class body.")?;
    
        Ok(())
    }

    fn expression_statement(&mut self) -> Result<(), Problem> {
        self.expression()?;
        self.consume(TokenType::SemiColon, "Jparser: Expect ';' after expression.")?;
    
        Ok(())
    }
    
    fn expression(&mut self) -> Result<(), Problem> {
        // println!("Expression");
        Ok(())
    }
}
