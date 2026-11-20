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

const EXERCISE_02_27:i32=2;
const EXERCISE_02_29:i32=2;
const EXERCISE_02_30:i32=2;
const EXERCISE_02_31:i32=2;
const EXERCISE_02_32:i32=2;
const EXERCISE_02_33:i32=2;
const EXERCISE_02_34:i32=2;
const EXERCISE_02_35:i32=2;
const EXERCISE_02_36:i32=2;
const EXERCISE_02_37:i32=2;
const EXERCISE_02_38:i32=2;
const EXERCISE_02_39:i32=2;
const EXERCISE_02_40:i32=2;
const EXERCISE_02_41:i32=2;
const EXERCISE_02_42:i32=2;
const EXERCISE_02_43:i32=2;
const EXERCISE_02_44:i32=2;
const EXERCISE_02_45:i32=2;
const EXERCISE_02_46:i32=2;
const EXERCISE_02_47:i32=2;
const EXERCISE_02_48:i32=2;
const EXERCISE_02_49:i32=2;
const EXERCISE_02_50:i32=2;
const EXERCISE_02_51:i32=2;
const EXERCISE_02_52:i32=2;
const EXERCISE_02_53:i32=2;
const EXERCISE_02_54:i32=2;
const EXERCISE_02_55:i32=2;
const EXERCISE_02_56:i32=2;
const EXERCISE_02_57:i32=2;
const EXERCISE_02_58:i32=2;
const EXERCISE_02_59:i32=2;
const EXERCISE_02_60:i32=2;
const EXERCISE_02_61:i32=2;
const EXERCISE_02_62:i32=2;
const EXERCISE_02_63:i32=2;
const EXERCISE_02_64:i32=2;
const EXERCISE_02_65:i32=2;
const EXERCISE_02_66:i32=2;
const EXERCISE_02_67:i32=2;
const EXERCISE_02_68:i32=2;
const EXERCISE_02_69:i32=2;
const EXERCISE_02_70:i32=2;
const EXERCISE_02_71:i32=2;
const EXERCISE_02_72:i32=2;
const EXERCISE_02_73:i32=2;
const EXERCISE_02_74:i32=2;
const EXERCISE_02_75:i32=2;
const EXERCISE_02_76:i32=2;
const EXERCISE_02_77:i32=2;
const EXERCISE_02_78:i32=2;
const EXERCISE_02_79:i32=2;
