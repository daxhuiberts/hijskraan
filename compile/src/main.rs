use cranelift_codegen::ir::{AbiParam, InstBuilder};
use cranelift_codegen::Context;
use cranelift_codegen::ir::types::I32;
use cranelift_codegen::settings::{self, Flags};
use cranelift_frontend::{FunctionBuilder, FunctionBuilderContext};
use cranelift_module::{Linkage, Module};
use cranelift_object::{ObjectBuilder, ObjectModule};

struct Compiler {
    module: ObjectModule,
    context: Context,
    function_builder_context: FunctionBuilderContext,
}

impl Compiler {
    fn new() -> Self {
        let isa_builder = cranelift_native::builder().unwrap();

        let flag_builder = settings::builder();
        let flags = Flags::new(flag_builder);
        let isa = isa_builder.finish(flags).unwrap();

        let object_builder = ObjectBuilder::new(
            isa,
            "".to_string(),
            cranelift_module::default_libcall_names(),
        )
        .unwrap();
        let module = ObjectModule::new(object_builder);
        let context = module.make_context();
        let function_builder_context = FunctionBuilderContext::new();

        Self {
            module,
            context,
            function_builder_context,
        }
    }

    fn function(
        &mut self,
        name: &str,
        params: Vec<AbiParam>,
        return_: AbiParam,
        f: fn(&mut FunctionBuilder),
    ) {
        self.context.func.signature.params = params;
        self.context.func.signature.returns = vec![return_];

        let func_id = self
            .module
            .declare_function(name, Linkage::Export, &self.context.func.signature)
            .unwrap();

        let mut builder =
            FunctionBuilder::new(&mut self.context.func, &mut self.function_builder_context);

        let block0 = builder.create_block();
        builder.append_block_params_for_function_params(block0);
        builder.switch_to_block(block0);
        builder.seal_block(block0);

        f(&mut builder);

        builder.finalize();

        self.context.set_disasm(true);

        self.module
            .define_function(func_id, &mut self.context)
            .unwrap();

        let compiled = self.context.compiled_code().unwrap();

        println!("{}", &self.context.func);
        println!("{}", compiled.disasm.as_ref().unwrap());

        self.module.clear_context(&mut self.context);
    }

    fn finish(self, filename: &str) {
        let object_product = self.module.finish();
        let result = object_product.emit().unwrap();
        std::fs::write(filename, result).unwrap();
    }
}

fn main() {
    let mut compiler = Compiler::new();

    compiler.function("magic", vec![], AbiParam::new(I32), |builder| {
        let res = builder.ins().iconst(I32, 42);
        builder.ins().return_(&[res]);
    });

    compiler.function(
        "double",
        vec![AbiParam::new(I32)],
        AbiParam::new(I32),
        |builder| {
            let a = builder.block_params(builder.current_block().unwrap())[0];
            let b = builder.ins().iconst(I32, 2);
            let res = builder.ins().imul(a, b);
            builder.ins().return_(&[res]);
        },
    );

    compiler.finish("magic.o");
}
