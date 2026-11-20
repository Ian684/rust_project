#[derive(Clone,Debug)]
enum Exercise40State{Idle,Running,Finished}

struct Exercise40Record{id:usize,label:String,score:i32,state:Exercise40State}

impl Exercise40Record{
    fn grade(&self)->char{
        match self.score{95..=100=>'A',85..=94=>'B',70..=84=>'C',_=>'D'}
    }
}

fn exercise_40_operation(values:&[i32])->i64{
    values.iter().enumerate().map(|(index,value)|(*value as i64)*(index as i64+40)).sum()
}

fn main(){
    println!("Exercise 40: other");
    let records=[
        Exercise40Record{id:1,label:"Ada".into(),score:98,state:Exercise40State::Finished},
        Exercise40Record{id:2,label:"Linus".into(),score:86,state:Exercise40State::Running},
        Exercise40Record{id:3,label:"Grace".into(),score:73,state:Exercise40State::Idle},
    ];
    for record in &records{
        println!("{} {} {} {:?}",record.id,record.label,record.grade(),record.state);
    }
    println!("operation={}",exercise_40_operation(&[12,7,19,3,25]));
}

const EXERCISE_40_27:i32=40;
const EXERCISE_40_29:i32=40;
const EXERCISE_40_30:i32=40;
const EXERCISE_40_31:i32=40;
const EXERCISE_40_32:i32=40;
const EXERCISE_40_33:i32=40;
const EXERCISE_40_34:i32=40;
const EXERCISE_40_35:i32=40;
const EXERCISE_40_36:i32=40;
const EXERCISE_40_37:i32=40;
const EXERCISE_40_38:i32=40;
const EXERCISE_40_39:i32=40;
const EXERCISE_40_40:i32=40;
const EXERCISE_40_41:i32=40;
const EXERCISE_40_42:i32=40;
const EXERCISE_40_43:i32=40;
const EXERCISE_40_44:i32=40;
const EXERCISE_40_45:i32=40;
const EXERCISE_40_46:i32=40;
const EXERCISE_40_47:i32=40;
const EXERCISE_40_48:i32=40;
const EXERCISE_40_49:i32=40;
const EXERCISE_40_50:i32=40;
const EXERCISE_40_51:i32=40;
const EXERCISE_40_52:i32=40;
const EXERCISE_40_53:i32=40;
const EXERCISE_40_54:i32=40;
const EXERCISE_40_55:i32=40;
const EXERCISE_40_56:i32=40;
const EXERCISE_40_57:i32=40;
const EXERCISE_40_58:i32=40;
const EXERCISE_40_59:i32=40;
const EXERCISE_40_60:i32=40;
const EXERCISE_40_61:i32=40;
const EXERCISE_40_62:i32=40;
const EXERCISE_40_63:i32=40;
const EXERCISE_40_64:i32=40;
const EXERCISE_40_65:i32=40;
const EXERCISE_40_66:i32=40;
const EXERCISE_40_67:i32=40;
const EXERCISE_40_68:i32=40;
const EXERCISE_40_69:i32=40;
const EXERCISE_40_70:i32=40;
const EXERCISE_40_71:i32=40;
const EXERCISE_40_72:i32=40;
const EXERCISE_40_73:i32=40;
const EXERCISE_40_74:i32=40;
const EXERCISE_40_75:i32=40;
const EXERCISE_40_76:i32=40;
const EXERCISE_40_77:i32=40;
const EXERCISE_40_78:i32=40;
const EXERCISE_40_79:i32=40;
