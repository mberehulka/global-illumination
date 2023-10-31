use math::{Vec4, Vec2, Vec3};

use crate::{object::Object, texture::Texture, camera::Camera, dir_light::DirectionalLight};

#[inline(always)]
pub fn clear(pixels: &mut [u8]) {
    let l = pixels.len();
    let mut i = 0;
    while i < l {
        pixels[i] = 0;
        pixels[i+1] = 0;
        pixels[i+2] = 0;
        pixels[i+3] = 255;
        i += 4;
    }
}
#[inline(always)]
pub fn draw(
    width: i32, height: i32,
    pixels: &mut [u8],
    zbuffer: &mut [f32],
    object: &Object,
    camera: &Camera,
    dir_light: &DirectionalLight
) {
    for [a, b, c] in object.vertices.iter() {
        let ap = object.transform * a.position;
        let bp = object.transform * b.position;
        let cp = object.transform * c.position;
        let an = (object.transform.rotation * a.normal).normalized();
        
        if an.dot(camera.position - ap) <= 0. { continue }

        let dp = an.dot(dir_light.direction);
        
        let intensity = (1. - dp * 0.75).into();
        
        project_triangle(
            width, height,
            pixels,
            zbuffer,
            camera.mat * ap.extend(1.),
            camera.mat * bp.extend(1.),
            camera.mat * cp.extend(1.),
            a.uv, b.uv, c.uv,
            object.texture,
            &*object.shadow_map.borrow(),
            intensity
        )
    }
}
#[inline(always)]
fn project_triangle(
    width: i32, height: i32,
    pixels: &mut [u8],
    zbuffer: &mut [f32],
    mut a: Vec4, mut b: Vec4, mut c: Vec4,
    mut auv: Vec2, mut buv: Vec2, mut cuv: Vec2,
    diffuse: &Texture,
    shadow_map: &Texture,
    color: Vec3
) {
    a.x /= a.w;  a.y /= a.w;  a.z /= a.w;
    b.x /= b.w;  b.y /= b.w;  b.z /= b.w;
    c.x /= c.w;  c.y /= c.w;  c.z /= c.w;
    auv /= a.w;  buv /= b.w;  cuv /= c.w;
    a.w = 1./a.w;  b.w = 1./b.w;  c.w = 1./c.w;
    if a.w <= 0. || b.w <= 0. || c.w <= 0. { return }
    raster_triangle(
        width, height,
        pixels,
        zbuffer,
        ((a.x + 1.) * 0.5 * width as f32) as i32,
        ((a.y + 1.) * 0.5 * height as f32) as i32,
        ((b.x + 1.) * 0.5 * width as f32) as i32,
        ((b.y + 1.) * 0.5 * height as f32) as i32,
        ((c.x + 1.) * 0.5 * width as f32) as i32,
        ((c.y + 1.) * 0.5 * height as f32) as i32,
        Vec3::new(a.z, b.z, c.z),
        Vec3::new(a.w, b.w, c.w),
        Vec3::new(auv.x, buv.x, cuv.x),
        Vec3::new(auv.y, buv.y, cuv.y),
        diffuse,
        shadow_map,
        color
    )
}
#[inline(always)]
fn raster_triangle(
    width: i32, height: i32,
    pixels: &mut [u8],
    zbuffer: &mut [f32],
    ax: i32, ay: i32,
    bx: i32, by: i32,
    cx: i32, cy: i32,
    z: Vec3,
    w: Vec3,
    uvx: Vec3,
    uvy: Vec3,
    diffuse: &Texture,
    shadow_map: &Texture,
    color: Vec3
) {
    let max_width = width - 1;
    let max_height = height - 1;
    
    let minx = max_width.min(ax).max(0).min(bx).max(0).min(cx).max(0);
    let mut miny = max_height.min(ay).max(0).min(by).max(0).min(cy).max(0);
    let maxx = 0.max(ax).min(max_width).max(bx).min(max_width).max(cx).min(max_width);
    let maxy = 0.max(ay).min(max_height).max(by).min(max_height).max(cy).min(max_height);

    let l1x = cx as f32 - ax as f32;
    let l1y = bx as f32 - ax as f32;
    let mut l1z;
    let l2x = cy as f32 - ay as f32;
    let l2y = by as f32 - ay as f32;
    let mut l2z;
    
    let mut ux; let mut uy;
    let uz = (l1x * l2y) - (l1y * l2x);
    if uz.abs() < 1. { return }

    let mut i = (miny * width) as usize * 4;
    let mut _i = (miny * width) as usize;
    let mut x;
    let line_width = (max_width - maxx) as usize * 4;
    let _line_width = (max_width - maxx) as usize;
    let line_offset = minx as usize * 4;
    let sm_size = shadow_map.size - 1.;
    let df_size = diffuse.size - 1.;
    let mut baryc = Vec3::default();
    
    while miny <= maxy {
        l2z = ay as f32 - miny as f32;
        x = minx;
        i += line_offset;
        _i += minx as usize;
        while x <= maxx {
            l1z = ax as f32 - x as f32;
            ux = (l1y * l2z) - (l1z * l2y);
            uy = (l1z * l2x) - (l1x * l2z);

            baryc.x = 1.-(ux+uy)/uz;
            if baryc.x < 0. { x += 1; i += 4; _i += 1; continue }

            baryc.y = uy/uz;
            if baryc.y < 0. { x += 1; i += 4; _i += 1; continue }

            baryc.z = ux/uz;
            if baryc.z < 0. { x += 1; i += 4; _i += 1; continue }

            let z = z.dot(baryc);
            if zbuffer[_i] >= z {
                let w = w.dot(baryc);
                let smuv = Vec2::new(uvx.dot(baryc), uvy.dot(baryc)) * sm_size / w;
                let dfuv = Vec2::new(uvx.dot(baryc), uvy.dot(baryc)) * df_size / w;
                let tex_color =
                    shadow_map.pixels[smuv.y as usize][smuv.x as usize] *
                    diffuse.pixels[dfuv.y as usize][dfuv.x as usize] *
                    color * 255.;
                pixels[i    ] = tex_color.x as u8;
                pixels[i + 1] = tex_color.y as u8;
                pixels[i + 2] = tex_color.z as u8;
                zbuffer[_i] = z;
            }

            x += 1;
            i += 4;
            _i += 1;
        }
        miny += 1;
        i += line_width;
        _i += _line_width
    }
}