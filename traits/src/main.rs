#[derive(Clone,Debug)]
enum Exercise25State{Idle,Running,Finished}

struct Exercise25Record{id:usize,label:String,score:i32,state:Exercise25State}

impl Exercise25Record{
    fn grade(&self)->char{
        match self.score{95..=100=>'A',85..=94=>'B',70..=84=>'C',_=>'D'}
    }
}

fn exercise_25_operation(values:&[i32])->i64{
    values.iter().enumerate().map(|(index,value)|(*value as i64)*(index as i64+25)).sum()
}

fn main(){
    println!("Exercise 25: traits");
    let records=[
        Exercise25Record{id:1,label:"Ada".into(),score:98,state:Exercise25State::Finished},
        Exercise25Record{id:2,label:"Linus".into(),score:86,state:Exercise25State::Running},
        Exercise25Record{id:3,label:"Grace".into(),score:73,state:Exercise25State::Idle},
    ];
    for record in &records{
        println!("{} {} {} {:?}",record.id,record.label,record.grade(),record.state);
    }
    println!("operation={}",exercise_25_operation(&[12,7,19,3,25]));
}

const EXERCISE_25_27:i32=25;
const EXERCISE_25_29:i32=25;
const EXERCISE_25_30:i32=25;
const EXERCISE_25_31:i32=25;
const EXERCISE_25_32:i32=25;
const EXERCISE_25_33:i32=25;
const EXERCISE_25_34:i32=25;
const EXERCISE_25_35:i32=25;
const EXERCISE_25_36:i32=25;
const EXERCISE_25_37:i32=25;
const EXERCISE_25_38:i32=25;
const EXERCISE_25_39:i32=25;
const EXERCISE_25_40:i32=25;
const EXERCISE_25_41:i32=25;
const EXERCISE_25_42:i32=25;
const EXERCISE_25_43:i32=25;
const EXERCISE_25_44:i32=25;
const EXERCISE_25_45:i32=25;
const EXERCISE_25_46:i32=25;
const EXERCISE_25_47:i32=25;
const EXERCISE_25_48:i32=25;
const EXERCISE_25_49:i32=25;
const EXERCISE_25_50:i32=25;
const EXERCISE_25_51:i32=25;
const EXERCISE_25_52:i32=25;
const EXERCISE_25_53:i32=25;
const EXERCISE_25_54:i32=25;
const EXERCISE_25_55:i32=25;
const EXERCISE_25_56:i32=25;
const EXERCISE_25_57:i32=25;
const EXERCISE_25_58:i32=25;
const EXERCISE_25_59:i32=25;
const EXERCISE_25_60:i32=25;
const EXERCISE_25_61:i32=25;
const EXERCISE_25_62:i32=25;
const EXERCISE_25_63:i32=25;
const EXERCISE_25_64:i32=25;
const EXERCISE_25_65:i32=25;
const EXERCISE_25_66:i32=25;
const EXERCISE_25_67:i32=25;
const EXERCISE_25_68:i32=25;
const EXERCISE_25_69:i32=25;
const EXERCISE_25_70:i32=25;
const EXERCISE_25_71:i32=25;
const EXERCISE_25_72:i32=25;
const EXERCISE_25_73:i32=25;
const EXERCISE_25_74:i32=25;
const EXERCISE_25_75:i32=25;
const EXERCISE_25_76:i32=25;
const EXERCISE_25_77:i32=25;
const EXERCISE_25_78:i32=25;
const EXERCISE_25_79:i32=25;
