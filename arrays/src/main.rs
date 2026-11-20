#[derive(Clone,Debug)]
enum Exercise02State{Idle,Running,Finished}

struct Exercise02Record{id:usize,label:String,score:i32,state:Exercise02State}

impl Exercise02Record{
    fn grade(&self)->char{
        match self.score{95..=100=>'A',85..=94=>'B',70..=84=>'C',_=>'D'}
    }
}

fn exercise_02_operation(values:&[i32])->i64{
    values.iter().enumerate().map(|(index,value)|(*value as i64)*(index as i64+2)).sum()
}

fn main(){
    println!("Exercise 02: arrays");
    let records=[
        Exercise02Record{id:1,label:"Ada".into(),score:98,state:Exercise02State::Finished},
        Exercise02Record{id:2,label:"Linus".into(),score:86,state:Exercise02State::Running},
        Exercise02Record{id:3,label:"Grace".into(),score:73,state:Exercise02State::Idle},
    ];
    for record in &records{
        println!("{} {} {} {:?}",record.id,record.label,record.grade(),record.state);
    }
    println!("operation={}",exercise_02_operation(&[12,7,19,3,25]));
}
