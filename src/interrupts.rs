extern crate x86_64;
use x86_64::structures::idt::{InterruptDescriptorTable, ExceptionStackFrame};
use gdt;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe{
            idt.double_fault.set_handler_fn(double_fault_handler).set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
            
        }
        idt
    };
}

pub fn init_idt(){
    IDT.load();
}


// breakpoint exception handler. This is how debuggers work by throughing 
// a breakpoint exception then replacing the exception call with the original 
// stack instruction. x86-interrupt is the type of code that is run
extern "x86-interrupt" fn breakpoint_handler( stack_frame: &mut ExceptionStackFrame ){
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

// this is a double fault handler. it cautches the unhandled faults and 
// prevents triple faults which cause system resets and cannot be 
// recovered from.
extern "x86-interrupt" fn double_fault_handler( stack_frame: &mut ExceptionStackFrame, _error_code: u64){
    println!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
    loop{}
}
