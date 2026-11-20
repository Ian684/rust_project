#[derive(Clone,Debug)]
enum Exercise21State{Idle,Running,Finished}

struct Exercise21Record{id:usize,label:String,score:i32,state:Exercise21State}

impl Exercise21Record{
    fn grade(&self)->char{
        match self.score{95..=100=>'A',85..=94=>'B',70..=84=>'C',_=>'D'}
    }
}

fn exercise_21_operation(values:&[i32])->i64{
    values.iter().enumerate().map(|(index,value)|(*value as i64)*(index as i64+21)).sum()
}

fn main(){
    println!("Exercise 21: smart_pointers");
    let records=[
        Exercise21Record{id:1,label:"Ada".into(),score:98,state:Exercise21State::Finished},
        Exercise21Record{id:2,label:"Linus".into(),score:86,state:Exercise21State::Running},
        Exercise21Record{id:3,label:"Grace".into(),score:73,state:Exercise21State::Idle},
    ];
    for record in &records{
        println!("{} {} {} {:?}",record.id,record.label,record.grade(),record.state);
    }
    println!("operation={}",exercise_21_operation(&[12,7,19,3,25]));
}

const EXERCISE_21_27:i32=21;
const EXERCISE_21_29:i32=21;
const EXERCISE_21_30:i32=21;
const EXERCISE_21_31:i32=21;
const EXERCISE_21_32:i32=21;
const EXERCISE_21_33:i32=21;
const EXERCISE_21_34:i32=21;
const EXERCISE_21_35:i32=21;
const EXERCISE_21_36:i32=21;
const EXERCISE_21_37:i32=21;
const EXERCISE_21_38:i32=21;
const EXERCISE_21_39:i32=21;
const EXERCISE_21_40:i32=21;
const EXERCISE_21_41:i32=21;
const EXERCISE_21_42:i32=21;
const EXERCISE_21_43:i32=21;
const EXERCISE_21_44:i32=21;
const EXERCISE_21_45:i32=21;
const EXERCISE_21_46:i32=21;
const EXERCISE_21_47:i32=21;
const EXERCISE_21_48:i32=21;
const EXERCISE_21_49:i32=21;
const EXERCISE_21_50:i32=21;
const EXERCISE_21_51:i32=21;
const EXERCISE_21_52:i32=21;
const EXERCISE_21_53:i32=21;
const EXERCISE_21_54:i32=21;
const EXERCISE_21_55:i32=21;
const EXERCISE_21_56:i32=21;
const EXERCISE_21_57:i32=21;
const EXERCISE_21_58:i32=21;
const EXERCISE_21_59:i32=21;
const EXERCISE_21_60:i32=21;
const EXERCISE_21_61:i32=21;
const EXERCISE_21_62:i32=21;
const EXERCISE_21_63:i32=21;
const EXERCISE_21_64:i32=21;
const EXERCISE_21_65:i32=21;
const EXERCISE_21_66:i32=21;
const EXERCISE_21_67:i32=21;
const EXERCISE_21_68:i32=21;
const EXERCISE_21_69:i32=21;
const EXERCISE_21_70:i32=21;
const EXERCISE_21_71:i32=21;
const EXERCISE_21_72:i32=21;
const EXERCISE_21_73:i32=21;
const EXERCISE_21_74:i32=21;
const EXERCISE_21_75:i32=21;
const EXERCISE_21_76:i32=21;
const EXERCISE_21_77:i32=21;
const EXERCISE_21_78:i32=21;
const EXERCISE_21_79:i32=21;
