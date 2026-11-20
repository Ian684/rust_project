#[derive(Clone,Debug)]
enum Exercise05State{Idle,Running,Finished}

struct Exercise05Record{id:usize,label:String,score:i32,state:Exercise05State}

impl Exercise05Record{
    fn grade(&self)->char{
        match self.score{95..=100=>'A',85..=94=>'B',70..=84=>'C',_=>'D'}
    }
}

fn exercise_05_operation(values:&[i32])->i64{
    values.iter().enumerate().map(|(index,value)|(*value as i64)*(index as i64+5)).sum()
}

fn main(){
    println!("Exercise 05: channels");
    let records=[
        Exercise05Record{id:1,label:"Ada".into(),score:98,state:Exercise05State::Finished},
        Exercise05Record{id:2,label:"Linus".into(),score:86,state:Exercise05State::Running},
        Exercise05Record{id:3,label:"Grace".into(),score:73,state:Exercise05State::Idle},
    ];
    for record in &records{
        println!("{} {} {} {:?}",record.id,record.label,record.grade(),record.state);
    }
    println!("operation={}",exercise_05_operation(&[12,7,19,3,25]));
}

const EXERCISE_05_27:i32=5;
const EXERCISE_05_29:i32=5;
const EXERCISE_05_30:i32=5;
const EXERCISE_05_31:i32=5;
const EXERCISE_05_32:i32=5;
const EXERCISE_05_33:i32=5;
const EXERCISE_05_34:i32=5;
const EXERCISE_05_35:i32=5;
const EXERCISE_05_36:i32=5;
const EXERCISE_05_37:i32=5;
const EXERCISE_05_38:i32=5;
const EXERCISE_05_39:i32=5;
const EXERCISE_05_40:i32=5;
const EXERCISE_05_41:i32=5;
const EXERCISE_05_42:i32=5;
const EXERCISE_05_43:i32=5;
const EXERCISE_05_44:i32=5;
const EXERCISE_05_45:i32=5;
const EXERCISE_05_46:i32=5;
const EXERCISE_05_47:i32=5;
const EXERCISE_05_48:i32=5;
const EXERCISE_05_49:i32=5;
const EXERCISE_05_50:i32=5;
const EXERCISE_05_51:i32=5;
const EXERCISE_05_52:i32=5;
const EXERCISE_05_53:i32=5;
const EXERCISE_05_54:i32=5;
const EXERCISE_05_55:i32=5;
const EXERCISE_05_56:i32=5;
const EXERCISE_05_57:i32=5;
const EXERCISE_05_58:i32=5;
const EXERCISE_05_59:i32=5;
const EXERCISE_05_60:i32=5;
const EXERCISE_05_61:i32=5;
const EXERCISE_05_62:i32=5;
const EXERCISE_05_63:i32=5;
const EXERCISE_05_64:i32=5;
const EXERCISE_05_65:i32=5;
const EXERCISE_05_66:i32=5;
const EXERCISE_05_67:i32=5;
const EXERCISE_05_68:i32=5;
const EXERCISE_05_69:i32=5;
const EXERCISE_05_70:i32=5;
const EXERCISE_05_71:i32=5;
const EXERCISE_05_72:i32=5;
const EXERCISE_05_73:i32=5;
const EXERCISE_05_74:i32=5;
const EXERCISE_05_75:i32=5;
const EXERCISE_05_76:i32=5;
const EXERCISE_05_77:i32=5;
const EXERCISE_05_78:i32=5;
const EXERCISE_05_79:i32=5;
