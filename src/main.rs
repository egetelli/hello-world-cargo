fn main() {
    println!("Hello, world!");
    let mut x = 45;
    println!("The value of x is {}",x);
    let mut y = 60;
    println!("The value of x is {}",x);
    let mut f = 5.5;
    println!("The value of f is {}",f);

    let ege: f32 = 10.5;
    println!("The value of ege is {}",ege);


    let a = false;
    println!("The value of a is {}",a);


    if x>y{
        println!("x, y'den büyük");
    }
    else{
        println!("y, x'den büyük");
    }

    let mut a = 0;

    loop{
        a+=3;

        if a > 100{
            break;
        }
        println!("Sayımız = {}",a);
    }
    let mut b = 0;
    while b <=100{
        
        if b%10==0{
            println!("Bu Sayılar 10'un Katı : {}",b);
        }
        

        b = b+1;

    }

    for i in 1..30{
        if i%5 == 0{
            println!("Bu sayılar 5'in katı {}",i);
        }
    }

    let animals = vec!["Dog","Rabbit","Cat","Turtle","Zebra"];
    for (index,a) in animals.iter().enumerate(){
        println!("{} Numaralı hayvanın ismi {}",index,a);
    }




    let suankigun:Gunler = Gunler::Sali;

    

    match suankigun{
        Gunler::Pazartesi => println!("Günümüz Pazartesi"),
        Gunler::Sali => println!("Günümüz sali"),
        Gunler::Carsamba => println!("Günümüz carsamba"),
        Gunler::Persembe => println!("Günümüz persembe"),
        Gunler::Cuma => println!("Günümüz cuma"),
        Gunler::Cumartesi => println!("Günümüz cumartesi"),
        Gunler::Pazar => println!("Günümüz pazar"),
    }


    let mut line = String::new();
   println!("Enter your name :");
   let b1 = std::io::stdin().read_line(&mut line).unwrap();
   print!("Hello , {}", line);
   println!("no of bytes read , {}", b1);
}

enum Gunler{

    Pazartesi,
    Sali,
    Carsamba,
    Persembe,
    Cuma,
    Cumartesi,
    Pazar

}
