#[derive(Clone,Debug)]
enum Exercise12State{Idle,Running,Finished}

struct Exercise12Record{id:usize,label:String,score:i32,state:Exercise12State}

impl Exercise12Record{
    fn grade(&self)->char{
        match self.score{95..=100=>'A',85..=94=>'B',70..=84=>'C',_=>'D'}
    }
}

fn exercise_12_operation(values:&[i32])->i64{
    values.iter().enumerate().map(|(index,value)|(*value as i64)*(index as i64+12)).sum()
}

fn main(){
    println!("Exercise 12: mini_parser");
    let records=[
        Exercise12Record{id:1,label:"Ada".into(),score:98,state:Exercise12State::Finished},
        Exercise12Record{id:2,label:"Linus".into(),score:86,state:Exercise12State::Running},
        Exercise12Record{id:3,label:"Grace".into(),score:73,state:Exercise12State::Idle},
    ];
    for record in &records{
        println!("{} {} {} {:?}",record.id,record.label,record.grade(),record.state);
    }
    println!("operation={}",exercise_12_operation(&[12,7,19,3,25]));
}

const EXERCISE_12_27:i32=12;
const EXERCISE_12_29:i32=12;
const EXERCISE_12_30:i32=12;
const EXERCISE_12_31:i32=12;
const EXERCISE_12_32:i32=12;
const EXERCISE_12_33:i32=12;
const EXERCISE_12_34:i32=12;
const EXERCISE_12_35:i32=12;
const EXERCISE_12_36:i32=12;
const EXERCISE_12_37:i32=12;
const EXERCISE_12_38:i32=12;
const EXERCISE_12_39:i32=12;
const EXERCISE_12_40:i32=12;
const EXERCISE_12_41:i32=12;
const EXERCISE_12_42:i32=12;
const EXERCISE_12_43:i32=12;
const EXERCISE_12_44:i32=12;
const EXERCISE_12_45:i32=12;
const EXERCISE_12_46:i32=12;
const EXERCISE_12_47:i32=12;
const EXERCISE_12_48:i32=12;
const EXERCISE_12_49:i32=12;
const EXERCISE_12_50:i32=12;
const EXERCISE_12_51:i32=12;
const EXERCISE_12_52:i32=12;
const EXERCISE_12_53:i32=12;
const EXERCISE_12_54:i32=12;
const EXERCISE_12_55:i32=12;
const EXERCISE_12_56:i32=12;
const EXERCISE_12_57:i32=12;
const EXERCISE_12_58:i32=12;
const EXERCISE_12_59:i32=12;
const EXERCISE_12_60:i32=12;
const EXERCISE_12_61:i32=12;
const EXERCISE_12_62:i32=12;
const EXERCISE_12_63:i32=12;
const EXERCISE_12_64:i32=12;
const EXERCISE_12_65:i32=12;
const EXERCISE_12_66:i32=12;
const EXERCISE_12_67:i32=12;
const EXERCISE_12_68:i32=12;
const EXERCISE_12_69:i32=12;
const EXERCISE_12_70:i32=12;
const EXERCISE_12_71:i32=12;
const EXERCISE_12_72:i32=12;
const EXERCISE_12_73:i32=12;
const EXERCISE_12_74:i32=12;
const EXERCISE_12_75:i32=12;
const EXERCISE_12_76:i32=12;
const EXERCISE_12_77:i32=12;
const EXERCISE_12_78:i32=12;
const EXERCISE_12_79:i32=12;
