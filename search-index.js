var N=null,E="",T="t",U="u",searchIndex={};
var R=["information","apidata","status","string","joinedstation","response","result","error","Sends a get request to…","bikeshare","station_id","capacity","is_installed","is_renting","num_bikes_available","num_docks_available","is_returning","last_reported","try_from","try_into","borrow_mut","type_id","borrow","typeid","joinedstatus","formatter","serialize","deserialize","Information","JoinedStation","JoinedStatus","Returns a string that when prepended/appended to `unicode`…","register","appservice","InternalError","all_stations"];
searchIndex["bikeshare"]={"doc":E,"i":[[5,"join",R[9],"Joins the data from status and information into a hashmap…",N,[[[R[0]],[R[2]],[R[1],[R[0]]],[R[1],[R[2]]]],[["hashmap",[R[3],R[4]]],[R[3]],[R[4]]]]],[0,"api",E,E,N,N],[5,"get","bikeshare::api","Sends a get request to a specific url with the…",N,[[["str"]],[[R[6],[R[5],R[7]]],[R[7]],[R[5]]]]],[5,R[2],E,R[8],N,[[],[[R[6],["api",R[7]]],["api",[R[2]]],[R[7]]]]],[5,R[0],E,R[8],N,[[],[[R[6],["api",R[7]]],["api",[R[0]]],[R[7]]]]],[5,"status_with_root",E,"Sends a get request to `{root}/station_status.json`,…",N,[[["str"]],[[R[6],["api",R[7]]],["api",[R[2]]],[R[7]]]]],[5,"information_with_root",E,"Sends a get request to `{root}/station_information.json`,…",N,[[["str"]],[[R[6],["api",R[7]]],["api",[R[0]]],[R[7]]]]],[17,"BIKESHARE_SERVER",E,"Url of the remote api for non-tests",N,N],[17,"CLIENT_IDENTIFIER",E,"The Client-Identifier of this application.",N,N],[0,"types",R[9],E,N,N],[3,"Api","bikeshare::types","Both endpoints used by this application has the same top…",N,N],[12,"data",E,E,0,N],[12,"last_updated",E,E,0,N],[3,"ApiData",E,"Both endpoints used by this application has the same data…",N,N],[12,"stations",E,E,1,N],[3,"Status",E,"The data inside the station_status endpoint is modelled by…",N,N],[12,R[10],E,E,2,N],[12,R[12],E,E,2,N],[12,R[13],E,E,2,N],[12,R[14],E,E,2,N],[12,R[15],E,E,2,N],[12,R[16],E,E,2,N],[12,R[17],E,E,2,N],[3,R[28],E,"The data inside the station_information endpoint is…",N,N],[12,R[10],E,E,3,N],[12,"name",E,E,3,N],[12,"address",E,E,3,N],[12,"lat",E,E,3,N],[12,"lon",E,E,3,N],[12,R[11],E,E,3,N],[3,R[29],E,"The status struct contains \"dynamic\" content about…",N,N],[12,R[10],E,E,4,N],[12,"name",E,E,4,N],[12,"address",E,E,4,N],[12,"lat",E,E,4,N],[12,"lon",E,E,4,N],[12,R[11],E,E,4,N],[12,R[2],E,E,4,N],[3,R[30],E,"The status content of `JoinedStation`.",N,N],[12,R[12],E,E,5,N],[12,R[13],E,E,5,N],[12,R[14],E,E,5,N],[12,R[15],E,E,5,N],[12,R[16],E,E,5,N],[12,R[17],E,E,5,N],[17,"_IMPL_DESERIALIZE_FOR_Api",E,E,N,N],[17,"_IMPL_SERIALIZE_FOR_Api",E,E,N,N],[17,"_IMPL_DESERIALIZE_FOR_ApiData",E,E,N,N],[17,"_IMPL_SERIALIZE_FOR_ApiData",E,E,N,N],[17,"_IMPL_DESERIALIZE_FOR_Status",E,E,N,N],[17,"_IMPL_SERIALIZE_FOR_Status",E,E,N,N],[17,"_IMPL_DESERIALIZE_FOR_Information",E,E,N,N],[17,"_IMPL_SERIALIZE_FOR_Information",E,E,N,N],[17,"_IMPL_DESERIALIZE_FOR_JoinedStation",E,E,N,N],[17,"_IMPL_SERIALIZE_FOR_JoinedStation",E,E,N,N],[17,"_IMPL_DESERIALIZE_FOR_JoinedStatus",E,E,N,N],[17,"_IMPL_SERIALIZE_FOR_JoinedStatus",E,E,N,N],[11,"new",E,E,4,[[["option",[R[2]]],[R[2]],[R[0]]],["self"]]],[11,"docks_available_str",E,"Gets the number of available docks as a copy-on-write…",4,[[["str"],["self"],["cow",["str"]]],[["str"],["cow",["str"]]]]],[11,"bikes_available_str",E,"Gets the number of available bikes as a copy-on-write…",4,[[["str"],["self"],["cow",["str"]]],[["str"],["cow",["str"]]]]],[11,"into",E,E,0,[[],[U]]],[11,"from",E,E,0,[[[T]],[T]]],[11,R[18],E,E,0,[[[U]],[R[6]]]],[11,R[19],E,E,0,[[],[R[6]]]],[11,R[22],E,E,0,[[["self"]],[T]]],[11,R[20],E,E,0,[[["self"]],[T]]],[11,R[21],E,E,0,[[["self"]],[R[23]]]],[11,"into",E,E,1,[[],[U]]],[11,"from",E,E,1,[[[T]],[T]]],[11,R[18],E,E,1,[[[U]],[R[6]]]],[11,R[19],E,E,1,[[],[R[6]]]],[11,R[22],E,E,1,[[["self"]],[T]]],[11,R[20],E,E,1,[[["self"]],[T]]],[11,R[21],E,E,1,[[["self"]],[R[23]]]],[11,"into",E,E,2,[[],[U]]],[11,"from",E,E,2,[[[T]],[T]]],[11,R[18],E,E,2,[[[U]],[R[6]]]],[11,R[19],E,E,2,[[],[R[6]]]],[11,R[22],E,E,2,[[["self"]],[T]]],[11,R[20],E,E,2,[[["self"]],[T]]],[11,R[21],E,E,2,[[["self"]],[R[23]]]],[11,"into",E,E,3,[[],[U]]],[11,"from",E,E,3,[[[T]],[T]]],[11,R[18],E,E,3,[[[U]],[R[6]]]],[11,R[19],E,E,3,[[],[R[6]]]],[11,R[22],E,E,3,[[["self"]],[T]]],[11,R[20],E,E,3,[[["self"]],[T]]],[11,R[21],E,E,3,[[["self"]],[R[23]]]],[11,"into",E,E,4,[[],[U]]],[11,"from",E,E,4,[[[T]],[T]]],[11,R[18],E,E,4,[[[U]],[R[6]]]],[11,R[19],E,E,4,[[],[R[6]]]],[11,R[22],E,E,4,[[["self"]],[T]]],[11,R[20],E,E,4,[[["self"]],[T]]],[11,R[21],E,E,4,[[["self"]],[R[23]]]],[11,"into",E,E,5,[[],[U]]],[11,"from",E,E,5,[[[T]],[T]]],[11,R[18],E,E,5,[[[U]],[R[6]]]],[11,R[19],E,E,5,[[],[R[6]]]],[11,R[22],E,E,5,[[["self"]],[T]]],[11,R[20],E,E,5,[[["self"]],[T]]],[11,R[21],E,E,5,[[["self"]],[R[23]]]],[11,"eq",E,E,0,[[["self"],["api"]],["bool"]]],[11,"ne",E,E,0,[[["self"],["api"]],["bool"]]],[11,"eq",E,E,1,[[[R[1]],["self"]],["bool"]]],[11,"ne",E,E,1,[[[R[1]],["self"]],["bool"]]],[11,"eq",E,E,2,[[[R[2]],["self"]],["bool"]]],[11,"ne",E,E,2,[[[R[2]],["self"]],["bool"]]],[11,"eq",E,E,3,[[[R[0]],["self"]],["bool"]]],[11,"ne",E,E,3,[[[R[0]],["self"]],["bool"]]],[11,"eq",E,E,4,[[[R[4]],["self"]],["bool"]]],[11,"ne",E,E,4,[[[R[4]],["self"]],["bool"]]],[11,"eq",E,E,5,[[[R[24]],["self"]],["bool"]]],[11,"ne",E,E,5,[[[R[24]],["self"]],["bool"]]],[11,"fmt",E,E,0,[[["self"],[R[25]]],[R[6]]]],[11,"fmt",E,E,1,[[["self"],[R[25]]],[R[6]]]],[11,"fmt",E,E,2,[[["self"],[R[25]]],[R[6]]]],[11,"fmt",E,E,3,[[["self"],[R[25]]],[R[6]]]],[11,"fmt",E,E,4,[[["self"],[R[25]]],[R[6]]]],[11,"fmt",E,E,5,[[["self"],[R[25]]],[R[6]]]],[11,R[26],E,E,0,[[["self"],["__s"]],[R[6]]]],[11,R[26],E,E,1,[[["self"],["__s"]],[R[6]]]],[11,R[26],E,E,2,[[["self"],["__s"]],[R[6]]]],[11,R[26],E,E,3,[[["self"],["__s"]],[R[6]]]],[11,R[26],E,E,4,[[["self"],["__s"]],[R[6]]]],[11,R[26],E,E,5,[[["self"],["__s"]],[R[6]]]],[11,R[27],E,E,0,[[["__d"]],[R[6]]]],[11,R[27],E,E,1,[[["__d"]],[R[6]]]],[11,R[27],E,E,2,[[["__d"]],[R[6]]]],[11,R[27],E,E,3,[[["__d"]],[R[6]]]],[11,R[27],E,E,4,[[["__d"]],[R[6]]]],[11,R[27],E,E,5,[[["__d"]],[R[6]]]]],"p":[[3,"Api"],[3,"ApiData"],[3,"Status"],[3,R[28]],[3,R[29]],[3,R[30]]]};
searchIndex["sykl"]={"doc":E,"i":[[3,"Void","sykl",E,N,N],[5,"main",E,E,N,[[],[R[6]]]],[5,"print",E,E,N,[[],[["void"],[R[6],["void",R[7]]],[R[7]]]]],[5,"handle_error",E,"A \"best effort\" to print a usable error to the user in…",N,[[[R[7]]],[["i32"],[R[7]],[R[6],["i32",R[7]]]]]],[11,"into",E,E,0,[[],[U]]],[11,"from",E,E,0,[[[T]],[T]]],[11,R[18],E,E,0,[[[U]],[R[6]]]],[11,R[19],E,E,0,[[],[R[6]]]],[11,R[22],E,E,0,[[["self"]],[T]]],[11,R[20],E,E,0,[[["self"]],[T]]],[11,R[21],E,E,0,[[["self"]],[R[23]]]]],"p":[[3,"Void"]]};
searchIndex["sykl_formatting"]={"doc":E,"i":[[3,"Row","sykl_formatting","This struct represents a row in the pretty printed output.",N,N],[12,"name",E,E,0,N],[12,"docks",E,E,0,N],[12,"bikes",E,E,0,N],[5,"pretty_print_stations",E,"Prints each stations name, number of available docks and…",N,[[],[R[6]]]],[5,"pad",E,R[31],N,[[["str"],["usize"],["char"]],[R[3]]]],[5,"pad_into_buffer",E,R[31],N,[[["str"],[R[3]],["usize"],["char"]],[R[3]]]],[17,"EXTRA_PADDING",E,"The extra padding to add in addition to the length of the…",N,N],[17,"PLACEHOLDER",E,"The string to use in the case where no status information…",N,N],[17,"HEADER",E,"The headers used in the pretty print.",N,N],[11,"into",E,E,0,[[],[U]]],[11,"from",E,E,0,[[[T]],[T]]],[11,"to_owned",E,E,0,[[["self"]],[T]]],[11,"clone_into",E,E,0,[[["self"],[T]]]],[11,R[18],E,E,0,[[[U]],[R[6]]]],[11,R[19],E,E,0,[[],[R[6]]]],[11,R[22],E,E,0,[[["self"]],[T]]],[11,R[20],E,E,0,[[["self"]],[T]]],[11,R[21],E,E,0,[[["self"]],[R[23]]]],[11,"clone",E,E,0,[[["self"]],["row"]]]],"p":[[3,"Row"]]};
searchIndex["sykl_rest"]={"doc":E,"i":[[3,"station","sykl_rest",E,N,N],[3,R[35],E,E,N,N],[4,R[34],E,E,N,N],[13,"LockError",E,E,0,N],[5,"main",E,E,N,[[],[R[6]]]],[6,"Stations",E,E,N,N],[11,"into",E,E,1,[[],[U]]],[11,"from",E,E,1,[[[T]],[T]]],[11,R[18],E,E,1,[[[U]],[R[6]]]],[11,R[19],E,E,1,[[],[R[6]]]],[11,R[22],E,E,1,[[["self"]],[T]]],[11,R[20],E,E,1,[[["self"]],[T]]],[11,R[21],E,E,1,[[["self"]],[R[23]]]],[11,"vzip",E,E,1,[[],["v"]]],[11,"into",E,E,2,[[],[U]]],[11,"from",E,E,2,[[[T]],[T]]],[11,R[18],E,E,2,[[[U]],[R[6]]]],[11,R[19],E,E,2,[[],[R[6]]]],[11,R[22],E,E,2,[[["self"]],[T]]],[11,R[20],E,E,2,[[["self"]],[T]]],[11,R[21],E,E,2,[[["self"]],[R[23]]]],[11,"vzip",E,E,2,[[],["v"]]],[11,"into",E,E,0,[[],[U]]],[11,"from",E,E,0,[[[T]],[T]]],[11,"to_string",E,E,0,[[["self"]],[R[3]]]],[11,R[18],E,E,0,[[[U]],[R[6]]]],[11,R[19],E,E,0,[[],[R[6]]]],[11,R[22],E,E,0,[[["self"]],[T]]],[11,R[20],E,E,0,[[["self"]],[T]]],[11,R[21],E,E,0,[[["self"]],[R[23]]]],[11,"vzip",E,E,0,[[],["v"]]],[11,"from",E,E,0,[[["poisonerror"]],["self"]]],[11,"fmt",E,E,0,[[["self"],[R[25]]],[R[6]]]],[11,"fmt",E,E,0,[[["self"],[R[25]]],[R[6]]]],[11,"status_code",E,E,0,[[["self"]],["statuscode"]]],[11,R[32],E,E,1,[[[R[33]]]]],[11,R[32],E,E,2,[[[R[33]]]]]],"p":[[4,R[34]],[3,"station"],[3,R[35]]]};
initSearch(searchIndex);addSearchOptions(searchIndex);