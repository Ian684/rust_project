#[derive(Clone,Debug)]
enum Exercise16State{Idle,Running,Finished}

struct Exercise16Record{id:usize,label:String,score:i32,state:Exercise16State}

impl Exercise16Record{
    fn grade(&self)->char{
        match self.score{95..=100=>'A',85..=94=>'B',70..=84=>'C',_=>'D'}
    }
}

fn exercise_16_operation(values:&[i32])->i64{
    values.iter().enumerate().map(|(index,value)|(*value as i64)*(index as i64+16)).sum()
}

fn main(){
    println!("Exercise 16: queue");
    let records=[
        Exercise16Record{id:1,label:"Ada".into(),score:98,state:Exercise16State::Finished},
        Exercise16Record{id:2,label:"Linus".into(),score:86,state:Exercise16State::Running},
        Exercise16Record{id:3,label:"Grace".into(),score:73,state:Exercise16State::Idle},
    ];
    for record in &records{
        println!("{} {} {} {:?}",record.id,record.label,record.grade(),record.state);
    }
    println!("operation={}",exercise_16_operation(&[12,7,19,3,25]));
}

const EXERCISE_16_27:i32=16;
const EXERCISE_16_29:i32=16;
const EXERCISE_16_30:i32=16;
const EXERCISE_16_31:i32=16;
const EXERCISE_16_32:i32=16;
const EXERCISE_16_33:i32=16;
const EXERCISE_16_34:i32=16;
const EXERCISE_16_35:i32=16;
const EXERCISE_16_36:i32=16;
const EXERCISE_16_37:i32=16;
const EXERCISE_16_38:i32=16;
const EXERCISE_16_39:i32=16;
const EXERCISE_16_40:i32=16;
const EXERCISE_16_41:i32=16;
const EXERCISE_16_42:i32=16;
const EXERCISE_16_43:i32=16;
const EXERCISE_16_44:i32=16;
const EXERCISE_16_45:i32=16;
const EXERCISE_16_46:i32=16;
const EXERCISE_16_47:i32=16;
const EXERCISE_16_48:i32=16;
const EXERCISE_16_49:i32=16;
const EXERCISE_16_50:i32=16;
const EXERCISE_16_51:i32=16;
const EXERCISE_16_52:i32=16;
const EXERCISE_16_53:i32=16;
const EXERCISE_16_54:i32=16;
const EXERCISE_16_55:i32=16;
const EXERCISE_16_56:i32=16;
const EXERCISE_16_57:i32=16;
const EXERCISE_16_58:i32=16;
const EXERCISE_16_59:i32=16;
const EXERCISE_16_60:i32=16;
const EXERCISE_16_61:i32=16;
const EXERCISE_16_62:i32=16;
const EXERCISE_16_63:i32=16;
const EXERCISE_16_64:i32=16;
const EXERCISE_16_65:i32=16;
const EXERCISE_16_66:i32=16;
const EXERCISE_16_67:i32=16;
const EXERCISE_16_68:i32=16;
const EXERCISE_16_69:i32=16;
const EXERCISE_16_70:i32=16;
const EXERCISE_16_71:i32=16;
const EXERCISE_16_72:i32=16;
const EXERCISE_16_73:i32=16;
const EXERCISE_16_74:i32=16;
const EXERCISE_16_75:i32=16;
const EXERCISE_16_76:i32=16;
const EXERCISE_16_77:i32=16;
const EXERCISE_16_78:i32=16;
const EXERCISE_16_79:i32=16;
