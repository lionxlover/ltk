//! Shader program compilation and management.

use ltk_core::LtkResult;

pub struct ShaderProgram { pub id: u32, pub name: String }

/// Built-in shader sources used by the GL backend.
pub mod sources {
    pub const RECT_VERT: &str = r#"
        #version 460 core
        layout(location=0) in vec2 a_pos;
        uniform mat4 u_mvp;
        void main() { gl_Position = u_mvp * vec4(a_pos, 0.0, 1.0); }
    "#;

    pub const RECT_FRAG: &str = r#"
        #version 460 core
        out vec4 frag_color;
        uniform vec4 u_color;
        void main() { frag_color = u_color; }
    "#;

    pub const ROUNDED_RECT_FRAG: &str = r#"
        #version 460 core
        out vec4 frag_color;
        uniform vec4 u_color;
        uniform vec2 u_size;
        uniform float u_radius;
        in vec2 v_local_pos;
        float sdRoundRect(vec2 p, vec2 b, float r) {
            vec2 q = abs(p) - b + r;
            return length(max(q,0.0)) + min(max(q.x,q.y),0.0) - r;
        }
        void main() {
            float d = sdRoundRect(v_local_pos - u_size*0.5, u_size*0.5, u_radius);
            float alpha = 1.0 - smoothstep(0.0, 1.5, d);
            frag_color = vec4(u_color.rgb, u_color.a * alpha);
        }
    "#;

    pub const TEXT_FRAG: &str = r#"
        #version 460 core
        out vec4 frag_color;
        uniform sampler2D u_glyph_atlas;
        uniform vec4 u_color;
        in vec2 v_uv;
        void main() {
            float sdf = texture(u_glyph_atlas, v_uv).r;
            float alpha = smoothstep(0.45, 0.55, sdf);
            frag_color = vec4(u_color.rgb, u_color.a * alpha);
        }
    "#;
}

pub fn compile(name: &str, _vert: &str, _frag: &str) -> LtkResult<ShaderProgram> {
    log::debug!("ltk-gl: compiling shader program '{name}'");
    Ok(ShaderProgram { id: 0, name: name.to_string() })
}
