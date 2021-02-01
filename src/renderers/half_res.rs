use slingshot::{
    ash::vk,
    backend::image::*,
    rg::{self, SimpleRenderPass},
};

pub fn extract_half_res_gbuffer_view_normal_rgba8(
    rg: &mut rg::RenderGraph,
    gbuffer: &rg::Handle<Image>,
) -> rg::Handle<Image> {
    let mut output_tex = rg.create(
        gbuffer
            .desc()
            .half_res()
            .usage(vk::ImageUsageFlags::empty())
            .format(vk::Format::R8G8B8A8_SNORM),
    );
    SimpleRenderPass::new_compute(
        rg.add_pass("extract view normal/2"),
        "/assets/shaders/extract_half_res_gbuffer_view_normal_rgba8.hlsl",
    )
    .read(gbuffer)
    .write(&mut output_tex)
    .constants((
        gbuffer.desc().extent_inv_extent_2d(),
        output_tex.desc().extent_inv_extent_2d(),
    ))
    .dispatch(output_tex.desc().extent);
    output_tex
}

pub fn extract_half_res_depth(
    rg: &mut rg::RenderGraph,
    depth: &rg::Handle<Image>,
) -> rg::Handle<Image> {
    let mut output_tex = rg.create(
        depth
            .desc()
            .half_res()
            .usage(vk::ImageUsageFlags::empty())
            .format(vk::Format::R32_SFLOAT),
    );
    SimpleRenderPass::new_compute(
        rg.add_pass("downscale r"),
        "/assets/shaders/downscale_r.hlsl",
    )
    .read_aspect(depth, vk::ImageAspectFlags::DEPTH)
    .write(&mut output_tex)
    .constants((
        depth.desc().extent_inv_extent_2d(),
        output_tex.desc().extent_inv_extent_2d(),
    ))
    .dispatch(output_tex.desc().extent);
    output_tex
}
