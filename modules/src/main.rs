#[derive(Clone,Debug)]
enum Exercise13State{Idle,Running,Finished}

struct Exercise13Record{id:usize,label:String,score:i32,state:Exercise13State}

impl Exercise13Record{
    fn grade(&self)->char{
        match self.score{95..=100=>'A',85..=94=>'B',70..=84=>'C',_=>'D'}
    }
}

fn exercise_13_operation(values:&[i32])->i64{
    values.iter().enumerate().map(|(index,value)|(*value as i64)*(index as i64+13)).sum()
}

fn main(){
    println!("Exercise 13: modules");
    let records=[
        Exercise13Record{id:1,label:"Ada".into(),score:98,state:Exercise13State::Finished},
        Exercise13Record{id:2,label:"Linus".into(),score:86,state:Exercise13State::Running},
        Exercise13Record{id:3,label:"Grace".into(),score:73,state:Exercise13State::Idle},
    ];
    for record in &records{
        println!("{} {} {} {:?}",record.id,record.label,record.grade(),record.state);
    }
    println!("operation={}",exercise_13_operation(&[12,7,19,3,25]));
}
