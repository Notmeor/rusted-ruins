
use array2d::*;
use common::basic::TILE_SIZE_I;
use common::objholder::*;
use cairo::Context;
use gdk::prelude::ContextExt;
use pixbuf_holder::PixbufHolder;
use edit_map::EditingMap;

/// Draw tiles and objects on map
pub fn draw_map(cr: &Context, map: &EditingMap, pbh: &PixbufHolder, width: i32, height: i32) {
    let tile_nx = width / TILE_SIZE_I + 1;
    let tile_ny = height / TILE_SIZE_I + 1;

    // Clear drawing area
    cr.set_source_rgb(0.5, 0.5, 0.5);
    cr.paint();

    for iy in 0..tile_ny {
        for ix in 0..tile_nx {
            if ix >= map.width as i32 || iy >= map.height as i32 { continue; }
            let p = Vec2d::new(ix, iy);
            
            cr.set_source_pixbuf(pbh.get(map.tile[p]),
                                 (ix * TILE_SIZE_I) as f64,
                                 (iy * TILE_SIZE_I) as f64);
            cr.paint();
        }
    }
    
}

