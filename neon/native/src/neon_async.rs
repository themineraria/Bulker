use neon::prelude::*;

/*Used to execute a rust function asynchronously*/
pub struct Async {
    //can take a lambda that moves vars in it context so that the number of arg is still 0 and the functions in the lambda can have different signatures
    pub lambda: Box<dyn Fn() -> Result<String, String> + Send + Sync>,
}

impl Task for Async { //see https://neon-bindings.com/docs/async
    type Output = String;
    type Error = String;
    type JsEvent = JsString;
    fn perform(&self) -> Result<Self::Output, Self::Error> { //function to compute asynchronously
        (self.lambda)() //execute the lambda
    }
    fn complete(self, mut cx: TaskContext, result: Result<Self::Output, Self::Error>) -> JsResult<Self::JsEvent> { //cast the type for the JS callback    to get it
        match result { //"cast" the type Output to JsEvent
            Ok(x) => Ok(cx.string(x)),
            Err(e) => cx.throw_error(e),
        }
    }
}
