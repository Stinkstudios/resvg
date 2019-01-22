// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

// external
use cairo;
use kurbo;
use piet;

// self
use super::prelude::*;


pub fn draw(
    _tree: &usvg::Tree,
    path: &usvg::Path,
    _opt: &Options,
    _layers: &mut CairoLayers,
    cr: &mut cairo::Context,
) -> Rect {
    let bez_path = convert_path(&path.segments);

    let mut rc = CairoRenderContext::new(cr);
    let bbox = utils::path_bbox(&path.segments, None, &usvg::Transform::default());

    if path.visibility != usvg::Visibility::Visible {
        return bbox;
    }

    if let Some(ref fill) = path.fill {
        if let usvg::Paint::Color(color) = fill.paint {
            let brush = rc.solid_brush(to_piet_color(color, fill.opacity)).unwrap();
            let rule = match fill.rule {
                usvg::FillRule::NonZero => piet::FillRule::NonZero,
                usvg::FillRule::EvenOdd => piet::FillRule::EvenOdd,
            };

            rc.fill(&bez_path, &brush, rule).unwrap();
        }
    }

    if let Some(ref stroke) = path.stroke {
        if let usvg::Paint::Color(color) = stroke.paint {
            let brush = rc.solid_brush(to_piet_color(color, stroke.opacity)).unwrap();
            let mut style = piet::StrokeStyle::new();

            style.line_cap = Some(match stroke.linecap {
                usvg::LineCap::Butt => piet::LineCap::Butt,
                usvg::LineCap::Round => piet::LineCap::Round,
                usvg::LineCap::Square => piet::LineCap::Square,
            });

            style.line_join = Some(match stroke.linejoin {
                usvg::LineJoin::Miter => piet::LineJoin::Miter,
                usvg::LineJoin::Round => piet::LineJoin::Round,
                usvg::LineJoin::Bevel => piet::LineJoin::Bevel,
            });

            if let Some(ref dasharray) = stroke.dasharray {
                style.dash = Some((dasharray.0.clone(), stroke.dashoffset as f64));
            }

            style.miter_limit = Some(stroke.miterlimit.value());

            rc.stroke(&bez_path, &brush, stroke.width.value(), Some(&style)).unwrap();
        }
    }

    rc.finish().unwrap();

    bbox
}

fn convert_path(
    segments: &[usvg::PathSegment],
) -> kurbo::BezPath {
    let mut path = kurbo::BezPath::new();

    for segment in segments {
        match *segment {
            usvg::PathSegment::MoveTo { x, y } => {
                path.moveto((x, y));
            }
            usvg::PathSegment::LineTo { x, y } => {
                path.lineto((x, y));
            }
            usvg::PathSegment::CurveTo { x1, y1, x2, y2, x, y } => {
                path.curveto((x1, y1), (x2, y2), (x, y));
            }
            usvg::PathSegment::ClosePath => {
                path.closepath();
            }
        }
    }

    path
}

fn to_piet_color(c: usvg::Color, opacity: usvg::Opacity) -> u32 {
    let r = c.red as u32;
    let g = c.green as u32;
    let b = c.blue as u32;
    let a = (*opacity * 255.0) as u32;

    ((r & 0xff) << 24) | ((g & 0xff) << 16) | ((b & 0xff) << 8) | (a & 0xff)
}
