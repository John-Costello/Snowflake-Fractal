#![allow(unused_parens)]

use nannou::prelude::*;

const WIDTH:u32=800;
const HEIGHT:u32=800;

fn main() {
    nannou::app(model)     			
	        .size(WIDTH, HEIGHT)
			.update(update)		
			.run();
}

struct Model {
	frame_counter:i8,
    iteration_order: u8,
	recursion_length_limit:f32,
}

fn model(app: &App) -> Model {
	app.set_loop_mode(LoopMode::rate_fps(2.0));
    let window_id = app
        .new_window()
        .view(view)
        .raw_event(raw_window_event)
        .build()
        .unwrap();
		app.set_loop_mode(LoopMode::rate_fps(2.0));
    let _window = app.window(window_id).unwrap();  
	app.set_loop_mode(LoopMode::rate_fps(2.0));
    let model:Model = Model {
		frame_counter:-1,
        iteration_order:0,
        recursion_length_limit:729.0,    		
    };	
	return model;
}

fn update(_app: &App, model: &mut Model, _update: Update) {
	model.frame_counter+=1;
	if(model.frame_counter>=30)
	{   
	    model.frame_counter=0; 
        model.iteration_order+=1;
	    if(model.iteration_order>=5){   model.iteration_order=0;   }
	    if(model.iteration_order==0){   model.recursion_length_limit=729.0;   }
        else if(model.iteration_order==1){   model.recursion_length_limit=243.0;   }
        else if(model.iteration_order==2){   model.recursion_length_limit=81.0;   }
        else if(model.iteration_order==3){   model.recursion_length_limit=27.0;   }
        else if(model.iteration_order==4){   model.recursion_length_limit=9.0;   }		
	}	
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
	draw.background().color( rgb(212.0/255.0, 212.0/255.0, 212.0/255.0) );
	
	koch_fractal(300.0, -170.0, -300.0, -170.0, model.recursion_length_limit, & draw);
    koch_fractal(-300.0, -170.0, 0.0, 350.0, model.recursion_length_limit, & draw);
    koch_fractal(0.0, 350.0,  300.0, -170.0, model.recursion_length_limit, & draw);
	
	draw.to_frame(app, &frame).unwrap();
}

fn koch_fractal(xa:f32, ya:f32, xd:f32, yd:f32, recursion_length_limit:f32, draw:& nannou::Draw)
{
	let len:f32=((xd-xa).pow(2.0)+(yd-ya).pow(2.0)).sqrt();
	if(len<recursion_length_limit)
	{
		draw.line()
		    .start(pt2(xa, ya))
			.end(pt2(xd, yd))
			.weight(1.0)
			.color(BLACK);
	}
	else
	{
		let xb:f32 = xa+(xd-xa)/3.0;
		let yb:f32 = ya+(yd-ya)/3.0;
		let xc:f32 = xa+(xd-xa)*2.0/3.0;
		let yc:f32 = ya+(yd-ya)*2.0/3.0;
		let xm:f32 = xa+(xd-xa)/2.0;
		let ym:f32 = ya+(yd-ya)/2.0;
		let sine_theta=(yd-ya)/len;
		let cosine_theta=(xd-xa)/len;
		let xk=xm-(len/3.0)*((3.0).sqrt()/2.0)*sine_theta;
		let yk=ym+(len/3.0)*((3.0).sqrt()/2.0)*cosine_theta;
		
		koch_fractal(xa,ya,xb,yb, recursion_length_limit, & draw);
		koch_fractal(xb,yb,xk,yk, recursion_length_limit, & draw);
		koch_fractal(xk,yk,xc,yc, recursion_length_limit, & draw);
		koch_fractal(xc,yc,xd,yd, recursion_length_limit, & draw);
	}
}

fn raw_window_event(app: &App, _model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    if let nannou::winit::event::WindowEvent::KeyboardInput { input, .. } = event {
        if let (Some(nannou::winit::event::VirtualKeyCode::F), true) =
            (input.virtual_keycode, input.state == nannou::winit::event::ElementState::Pressed)
        {
            let window = app.main_window();
            let fullscreen = window.fullscreen().is_some();
            window.set_fullscreen(!fullscreen);
        }
    }
} 