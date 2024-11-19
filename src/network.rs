use poloto::build;
use rustfft::{num_complex::Complex, FftPlanner};

pub fn am_plot() {
	let sample_rate = 44100.0;
	let duration = 1.0;
	let n_samples = (sample_rate * duration) as usize;

	let mut signal: Vec<Complex<f64>> = (0..n_samples)
		.map(|i| {
			let t = i as f64;
			let t = i as f64 / sample_rate;
			Complex::new(am(t), 0.0)
		})
		.collect();
	
	let mut planner = FftPlanner::new();
	let fft = planner.plan_fft_forward(n_samples);
	fft.process(&mut signal);

	dbg!(signal.len());
	let plots = poloto::plots!(
		build::plot("").line(
			(0..n_samples).map(|i| (i as f64, signal[i].norm()))
			// (40000..n_samples).map(|i| (i as f64, signal[i].norm()))
		)
	);

	poloto::frame_build()
		.data(poloto::plots!(poloto::build::origin(), plots))
		.map_xticks(|_| poloto::ticks::from_iter((40000..n_samples).map(|i| i as f64) ))
		.build_and_label(("AM spectrum", "周波数", "振幅密度"))
		.append_to(poloto::header().light_theme())
		.render_stdout();
}

fn am(t: f64) -> f64 {
	let a = 1.0;
	let omega = 440.0;
	let p = 10.0;
	let q = 20.0;
	let r = 30.0;
	let m = 0.5;

	let v: f64 = (omega*t).cos() + 0.5*m*([p,q,r].map(|x| ((omega + x)*t).cos() + ((omega - x)*t).cos()).iter().sum::<f64>());

	v * a
}