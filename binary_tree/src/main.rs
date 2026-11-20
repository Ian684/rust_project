#[derive(Clone,Debug)]
enum Exercise03State{Idle,Running,Finished}

struct Exercise03Record{id:usize,label:String,score:i32,state:Exercise03State}

impl Exercise03Record{
    fn grade(&self)->char{
        match self.score{95..=100=>'A',85..=94=>'B',70..=84=>'C',_=>'D'}
    }
}

fn exercise_03_operation(values:&[i32])->i64{
    values.iter().enumerate().map(|(index,value)|(*value as i64)*(index as i64+3)).sum()
}

fn main(){
    println!("Exercise 03: binary_tree");
    let records=[
        Exercise03Record{id:1,label:"Ada".into(),score:98,state:Exercise03State::Finished},
        Exercise03Record{id:2,label:"Linus".into(),score:86,state:Exercise03State::Running},
        Exercise03Record{id:3,label:"Grace".into(),score:73,state:Exercise03State::Idle},
    ];
    for record in &records{
        println!("{} {} {} {:?}",record.id,record.label,record.grade(),record.state);
    }
    println!("operation={}",exercise_03_operation(&[12,7,19,3,25]));
}
