use rutie::{RString, AnyObject, Float, Object, class, methods};
use strsim::{levenshtein, normalized_levenshtein};

class!(RBSimilary);

methods!(
    RBSimilary,
    _rtself,
    fn sim_calculate(s1: RString, s2:RString) -> AnyObject {
        _sim_calculate(s1.unwrap(), s2.unwrap())
    }
);

fn _sim_calculate(s1: RString, s2:RString) -> AnyObject{
    let result = normalized_levenshtein(s1.to_str(), s2.to_str());
    Float::new(result).try_convert_to::<AnyObject>().unwrap()
}