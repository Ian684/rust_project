#[derive(Clone,Debug)]
enum Exercise11State{Idle,Running,Finished}

struct Exercise11Record{id:usize,label:String,score:i32,state:Exercise11State}

impl Exercise11Record{
    fn grade(&self)->char{
        match self.score{95..=100=>'A',85..=94=>'B',70..=84=>'C',_=>'D'}
    }
}

fn exercise_11_operation(values:&[i32])->i64{
    values.iter().enumerate().map(|(index,value)|(*value as i64)*(index as i64+11)).sum()
}

fn main(){
    println!("Exercise 11: methods");
    let records=[
        Exercise11Record{id:1,label:"Ada".into(),score:98,state:Exercise11State::Finished},
        Exercise11Record{id:2,label:"Linus".into(),score:86,state:Exercise11State::Running},
        Exercise11Record{id:3,label:"Grace".into(),score:73,state:Exercise11State::Idle},
    ];
    for record in &records{
        println!("{} {} {} {:?}",record.id,record.label,record.grade(),record.state);
    }
    println!("operation={}",exercise_11_operation(&[12,7,19,3,25]));
}

const EXERCISE_11_27:i32=11;
const EXERCISE_11_29:i32=11;
const EXERCISE_11_30:i32=11;
const EXERCISE_11_31:i32=11;
const EXERCISE_11_32:i32=11;
const EXERCISE_11_33:i32=11;
const EXERCISE_11_34:i32=11;
const EXERCISE_11_35:i32=11;
const EXERCISE_11_36:i32=11;
const EXERCISE_11_37:i32=11;
const EXERCISE_11_38:i32=11;
const EXERCISE_11_39:i32=11;
const EXERCISE_11_40:i32=11;
const EXERCISE_11_41:i32=11;
const EXERCISE_11_42:i32=11;
const EXERCISE_11_43:i32=11;
const EXERCISE_11_44:i32=11;
const EXERCISE_11_45:i32=11;
const EXERCISE_11_46:i32=11;
const EXERCISE_11_47:i32=11;
const EXERCISE_11_48:i32=11;
const EXERCISE_11_49:i32=11;
const EXERCISE_11_50:i32=11;
const EXERCISE_11_51:i32=11;
const EXERCISE_11_52:i32=11;
const EXERCISE_11_53:i32=11;
const EXERCISE_11_54:i32=11;
const EXERCISE_11_55:i32=11;
const EXERCISE_11_56:i32=11;
const EXERCISE_11_57:i32=11;
const EXERCISE_11_58:i32=11;
const EXERCISE_11_59:i32=11;
const EXERCISE_11_60:i32=11;
const EXERCISE_11_61:i32=11;
const EXERCISE_11_62:i32=11;
const EXERCISE_11_63:i32=11;
const EXERCISE_11_64:i32=11;
const EXERCISE_11_65:i32=11;
const EXERCISE_11_66:i32=11;
const EXERCISE_11_67:i32=11;
const EXERCISE_11_68:i32=11;
const EXERCISE_11_69:i32=11;
const EXERCISE_11_70:i32=11;
const EXERCISE_11_71:i32=11;
const EXERCISE_11_72:i32=11;
const EXERCISE_11_73:i32=11;
const EXERCISE_11_74:i32=11;
const EXERCISE_11_75:i32=11;
const EXERCISE_11_76:i32=11;
const EXERCISE_11_77:i32=11;
const EXERCISE_11_78:i32=11;
const EXERCISE_11_79:i32=11;
