use std::fs::File;
use std::io::prelude::*;

pub struct HtmlObject {
    first: String,
    body: String,
    last: String,
    full_string: String,
}

impl HtmlObject {
    pub fn new() -> Self {
        Self {
            first: "".to_string(),
            body: "".to_string(),
            last: "".to_string(),
            full_string: "".to_string(),
        }
    }

    pub fn get_device(&mut self) {
        self.first = format!(
            r#"
async function main() 
{{
    const adapter = await navigator.gpu?.requestAdapter();
    const device = await adapter?.requestDevice();
    if (!device) 
    {{
        console.log("need a browser that supports WebGPU");
        return;
    }}
    else 
    {{
        console.log("good");
    }}
"#
        );
        self.last = format!(
            r#"
}}
main();
"#
        );
        self.full_string = format!("{}{}{}", self.first, self.body, self.last);
    }

    pub fn get_canvas(&mut self) {
        self.body = format!(
            r#"
    const canvas = document.querySelector('canvas');
    const context = canvas.getContext('webgpu');
    const presentationFormat = navigator.gpu.getPreferredCanvasFormat();
    context.configure({{
        device,
        format: presentationFormat,
    }});
"#
        );
        self.full_string = format!("{}{}{}", self.first, self.body, self.last);
        self.first = format!("{}{}", self.first, self.body);
    }

    pub fn create_shader_module_vertex(&mut self) {
        self.body = format!(
            r#"
    const vsModule = device.createShaderModule({{
    label: 'triangle shaders with uniforms',
    code: `
        struct OurStruct 
        {{
            color: vec4f,
            scale: vec2f,
            offset: vec2f,
        }};
 
        @group(0) @binding(0) var<uniform> ourStruct: OurStruct;

        @vertex
        fn vs(
            @builtin(vertex_index) vertexIndex : u32
                ) -> @builtin(position) vec4f 
        {{
            let pos = array(
                vec2f( 0.0,  0.5),  // top center
                vec2f(-0.5, -0.5),  // bottom left
                vec2f( 0.5, -0.5)   // bottom right
            );
 
            return vec4f( pos[vertexIndex] * ourStruct.scale + ourStruct.offset, 0.0, 1.0 );
        }}
    `,
    }});
        "#
        );

        self.full_string = format!("{}{}{}", self.first, self.body, self.last);
        self.first = format!("{}{}", self.first, self.body);
    }

    pub fn create_shader_module_fragment(&mut self) {
        self.body = format!(
            r#"
    const fsModule = device.createShaderModule({{
    label: 'triangle shaders with uniforms',
    code: ` 

        struct OurStruct 
        {{
            color: vec4f,
            scale: vec2f,
            offset: vec2f,
        }};       
 
        @group(0) @binding(0) var<uniform> ourStruct: OurStruct;
        
        @fragment 
        fn fs(
            //@builtin(position) pixelPosition: vec4f
        ) -> @location(0) vec4f 
        {{            
            return ourStruct.color;
        }}
    `,
    }});
        "#
        );

        self.full_string = format!("{}{}{}", self.first, self.body, self.last);
        self.first = format!("{}{}", self.first, self.body);
    }

    pub fn create_uniformBufferSize(&mut self) {
        self.body = format!(
            r#"
    const uniformBufferSize =
        4 * 4 + // color is 4 32bit floats (4bytes each)
        2 * 4 + // scale is 2 32bit floats (4bytes each)
        2 * 4;  // offset is 2 32bit floats (4bytes each)
    const uniformBuffer = device.createBuffer({{
        size: uniformBufferSize,
        usage: GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST,
    }});
    // create a typedarray to hold the values for the uniforms in JavaScript
    const uniformValues = new Float32Array(uniformBufferSize / 4);
    // offsets to the various uniform values in float32 indices
    const kColorOffset = 0;
    const kScaleOffset = 4;
    const kOffsetOffset = 6;
 
    uniformValues.set([0, 1, 0, 1], kColorOffset);        // set the color
    uniformValues.set([-0.5, -0.25], kOffsetOffset);      // set the offset
        "#
        );

        self.full_string = format!("{}{}{}", self.first, self.body, self.last);
        self.first = format!("{}{}", self.first, self.body);
    }

    pub fn create_render_pipeline(&mut self) 
    {
        self.body = format!(
            r#"
    const pipeline = device.createRenderPipeline(
    {{
        label: 'hardcoded checkerboard triangle pipeline',
        layout: 'auto',
        vertex: 
        {{
            module: vsModule,
            entryPoint: 'vs',
        }},
        fragment: 
        {{
            module: fsModule,
            entryPoint: 'fs',
            targets: [{{ format: presentationFormat }}],
        }},
    }});
        "#
        );

        self.full_string = format!("{}{}{}", self.first, self.body, self.last);
        self.first = format!("{}{}", self.first, self.body);
    }

    pub fn create_BindGroup(&mut self) 
    {
        self.body = format!(
            r#"
    const bindGroup = device.createBindGroup({{
    layout: pipeline.getBindGroupLayout(0),
    entries: [
        {{ binding: 0, resource: {{ buffer: uniformBuffer }} }},
        ],
    }});
        "#
        );

        self.full_string = format!("{}{}{}", self.first, self.body, self.last);
        self.first = format!("{}{}", self.first, self.body);
    }

    pub fn create_render_pass_descriptor(&mut self) 
    {
        self.body = format!(
            r#"
    const renderPassDescriptor = 
    {{
        label: 'our basic canvas renderPass',
        colorAttachments: [
        {{
            // view: <- to be filled out when we render
            clearValue: [0.3, 0.3, 0.3, 1],
            loadOp: 'clear',
            storeOp: 'store',
        }},
        ],
    }};  
        "#
        );

        self.full_string = format!("{}{}{}", self.first, self.body, self.last);
        self.first = format!("{}{}", self.first, self.body);
    }

    pub fn render(&mut self) 
    {
        self.body = format!(
            r#"
    function render() 
    {{
        // Set the uniform values in our JavaScript side Float32Array
        const aspect = canvas.width / canvas.height;
        uniformValues.set([0.5 / aspect, 0.5], kScaleOffset); // set the scale
 
        // copy the values from JavaScript to the GPU
        device.queue.writeBuffer(uniformBuffer, 0, uniformValues);


        // Get the current texture from the canvas context and
        // set it as the texture to render to.
        renderPassDescriptor.colorAttachments[0].view =
            context.getCurrentTexture().createView();
 
        // make a command encoder to start encoding commands
        const encoder = device.createCommandEncoder({{ label: 'our encoder' }});
 
        // make a render pass encoder to encode render specific commands
        const pass = encoder.beginRenderPass(renderPassDescriptor);
        pass.setPipeline(pipeline);
        pass.setBindGroup(0, bindGroup);
        pass.draw(3);  // call our vertex shader 3 times
        pass.end();
 
        const commandBuffer = encoder.finish();
        device.queue.submit([commandBuffer]);
    }}
 
    const observer = new ResizeObserver(entries => 
    {{
        for (const entry of entries) 
        {{
            const canvas = entry.target;
            const width = entry.contentBoxSize[0].inlineSize;
            const height = entry.contentBoxSize[0].blockSize;
            canvas.width = Math.max(1, Math.min(width, device.limits.maxTextureDimension2D));
            canvas.height = Math.max(1, Math.min(height, device.limits.maxTextureDimension2D));
            // re-render
            render();
        }}
    }});
    observer.observe(canvas);
        "#
        );

        self.full_string = format!("{}{}{}", self.first, self.body, self.last);
        self.first = format!("{}{}", self.first, self.body);
    }

    pub fn write_to_file(&self, file_name_with_extension: String) 
    {
        let mut file = File::create(file_name_with_extension).expect("Could not create the file");
        file.write_all(self.full_string.as_bytes())
            .expect("Could not write to the file");
    }

    pub fn write_to_file_index_js(&self) 
    {
        self.write_to_file(String::from("index.js"));
    }
}
