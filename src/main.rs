mod vector;

use crate::vector::Vector;

fn main() {
    let a = Vector(vec![9.0, 2.0, 3.0]);
    let b = Vector(vec![1.0, 9.0, 3.0]);

    println!("{:<30} {:?}", "Vector A", a);
    println!("{:<30} {:?}", "Vector B", b);
    println!("{:<30} {:.3}", "Norm of A", a.norm().unwrap_or(0.0));
    println!("{:<30} {:.3}", "Norm of B", b.norm().unwrap_or(0.0));
    println!(
        "{:<30} {:.3}",
        "Dot product (A · B)",
        a.dot_product(&b).unwrap_or(0.0)
    );
    println!(
        "{:<30} {:.3}",
        "Scalar projection of A on B",
        a.scalar_projection(&b).unwrap_or(0.0)
    );
    println!(
        "{:<30} {:.3}",
        "Cosine similarity (A, B)",
        a.cosine_similarity(&b).unwrap_or(0.0)
    );

    let f = Vector(vec![1.0, 2.0, 3.0]);
    let g = Vector(vec![4.0, 5.0, 6.0]);
    let h = Vector(vec![0.0, 0.0, 0.0]);

    println!();
    println!("{:<30} {:?}", "Vector F", f);
    println!("{:<30} {:?}", "Vector G", g);
    println!("{:<30} {:?}", "F + G", f.add(&g).unwrap());
    println!("{:<30} {:?}", "F - G", f.sub(&g).unwrap());
    println!("{:<30} {:.3}", "F · G", f.dot_product(&g).unwrap_or(0.0));
    println!("{:<30} {:.3}", "Norm of F", f.norm().unwrap_or(0.0));
    println!("{:<30} {:.3}", "Norm of G", g.norm().unwrap_or(0.0));
    println!("{:<30} {:?}", "F * G", f.mul(&g).unwrap());
    println!("{:<30} {:?}", "F / G", f.div(&g).unwrap());
    println!("{:<30} {}", "Length of F", f.len());
    println!("{:<30} {}", "Length of G", g.len());
    println!(
        "{:<30} {:.3}",
        "Cosine similarity (F, G)",
        f.cosine_similarity(&g).unwrap_or(0.0)
    );

    println!();
    println!("{:<30} {:?}", "Vector H", h);
    println!("{:<30} {:.3}", "Norm of H", h.norm().unwrap_or(0.0));
    println!("{:<30} {:.3}", "F · H", f.dot_product(&h).unwrap_or(0.0));
    println!(
        "{:<30} {}",
        "Scalar projection of F onto H",
        f.scalar_projection(&h)
            .map(|v| format!("{:.3}", v))
            .unwrap_or("Err".to_string())
    );
    println!(
        "{:<30} {}",
        "Cosine similarity (F, H)",
        f.cosine_similarity(&h)
            .map(|v| format!("{:.3}", v))
            .unwrap_or("Err".to_string())
    );
    println!("{:<30} {}", "Length of H", h.len());
}
