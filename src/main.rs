mod html_creator;
use html_creator::HtmlObject;

fn main() 
{
    println!("Hello, world!");
    let mut my_file = HtmlObject::new();
    my_file.get_device();
    my_file.write_to_file_index_js();
    my_file.get_canvas();
    my_file.write_to_file_index_js();
    my_file.create_shader_module_vertex();
    my_file.write_to_file_index_js();
    my_file.create_shader_module_fragment();
    my_file.write_to_file_index_js();
    my_file.create_uniformBufferSize();
    my_file.write_to_file_index_js();
    my_file.create_render_pipeline();
    my_file.write_to_file_index_js();
    my_file.create_BindGroup();
    my_file.write_to_file_index_js();
    my_file.create_render_pass_descriptor();
    my_file.write_to_file_index_js();
    my_file.render();
    my_file.write_to_file_index_js();
    
}
