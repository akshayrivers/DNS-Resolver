Name servers manage two kinds of data. The first kind of data held in
sets called zones; each zone is the complete database for a particular
"pruned" subtree of the domain space. This data is called
authoritative. A name server periodically checks to make sure that its
zones are up to date, and if not, obtains a new copy of updated zones

Mockapetris [Page 3]

RFC 1035 Domain Implementation and Specification November 1987

from master files stored locally or in another name server. The second
kind of data is cached data which was acquired by a local resolver.
This data may be incomplete, but improves the performance of the
retrieval process when non-local data is repeatedly accessed. Cached
data is eventually discarded by a timeout mechanism.

3.2. RR definitions

3.2.1. Format

All RRs have the same top level format shown below:

                                    1  1  1  1  1  1
      0  1  2  3  4  5  6  7  8  9  0  1  2  3  4  5
    +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    |                                               |
    /                                               /
    /                      NAME                     /
    |                                               |
    +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    |                      TYPE                     |
    +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    |                     CLASS                     |
    +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    |                      TTL                      |
    |                                               |
    +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    |                   RDLENGTH                    |
    +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--|
    /                     RDATA                     /
    /                                               /
    +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+

where:

NAME an owner name, i.e., the name of the node to which this
resource record pertains.

TYPE two octets containing one of the RR TYPE codes.

CLASS two octets containing one of the RR CLASS codes.

TTL a 32 bit signed integer that specifies the time interval
that the resource record may be cached before the source
of the information should again be consulted. Zero
values are interpreted to mean that the RR can only be
used for the transaction in progress, and should not be
cached. For example, SOA records are always distributed
with a zero TTL to prohibit caching. Zero values can
also be used for extremely volatile data.

RDLENGTH an unsigned 16 bit integer that specifies the length in
octets of the RDATA field.

Mockapetris [Page 11]

RFC 1035 Domain Implementation and Specification November 1987

RDATA a variable length string of octets that describes the
resource. The format of this information varies
according to the TYPE and CLASS of the resource record.

3.2.2. TYPE values

TYPE fields are used in resource records. Note that these types are a
subset of QTYPEs.

TYPE value and meaning

A 1 a host address

NS 2 an authoritative name server

MD 3 a mail destination (Obsolete - use MX)

MF 4 a mail forwarder (Obsolete - use MX)

CNAME 5 the canonical name for an alias

SOA 6 marks the start of a zone of authority

MB 7 a mailbox domain name (EXPERIMENTAL)

MG 8 a mail group member (EXPERIMENTAL)

MR 9 a mail rename domain name (EXPERIMENTAL)

NULL 10 a null RR (EXPERIMENTAL)

WKS 11 a well known service description

PTR 12 a domain name pointer

HINFO 13 host information

MINFO 14 mailbox or mail list information

MX 15 mail exchange

TXT 16 text strings

3.2.3. QTYPE values

QTYPE fields appear in the question part of a query. QTYPES are a
superset of TYPEs, hence all TYPEs are valid QTYPEs. In addition, the
following QTYPEs are defined:

Mockapetris [Page 12]

RFC 1035 Domain Implementation and Specification November 1987

AXFR 252 A request for a transfer of an entire zone

MAILB 253 A request for mailbox-related records (MB, MG or MR)

MAILA 254 A request for mail agent RRs (Obsolete - see MX)

-               255 A request for all records

  3.2.4. CLASS values

CLASS fields appear in resource records. The following CLASS mnemonics
and values are defined:

IN 1 the Internet

CS 2 the CSNET class (Obsolete - used only for examples in
some obsolete RFCs)

CH 3 the CHAOS class

HS 4 Hesiod [Dyer 87]

3.3. Standard RRs

The following RR definitions are expected to occur, at least
potentially, in all classes. In particular, NS, SOA, CNAME, and PTR
will be used in all classes, and have the same format in all classes.
Because their RDATA format is known, all domain names in the RDATA
section of these RRs may be compressed.

<domain-name> is a domain name represented as a series of labels, and
terminated by a label with zero length. <character-string> is a single
length octet followed by that number of characters. <character-string>
is treated as binary information, and can be up to 256 characters in
length (including the length octet).

4.1.1. Header section format

The header contains the following fields:

                                    1  1  1  1  1  1
      0  1  2  3  4  5  6  7  8  9  0  1  2  3  4  5
    +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    |                      ID                       |
    +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    |QR|   Opcode  |AA|TC|RD|RA|   Z    |   RCODE   |
    +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    |                    QDCOUNT                    |
    +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    |                    ANCOUNT                    |
    +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    |                    NSCOUNT                    |
    +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    |                    ARCOUNT                    |
    +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+

where:

ID A 16 bit identifier assigned by the program that
generates any kind of query. This identifier is copied
the corresponding reply and can be used by the requester
to match up replies to outstanding queries.

QR A one bit field that specifies whether this message is a
query (0), or a response (1).

OPCODE A four bit field that specifies kind of query in this
message. This value is set by the originator of a query
and copied into the response. The values are:

                0               a standard query (QUERY)

                1               an inverse query (IQUERY)

                2               a server status request (STATUS)

                3-15            reserved for future use

AA Authoritative Answer - this bit is valid in responses,
and specifies that the responding name server is an
authority for the domain name in question section.

                Note that the contents of the answer section may have
                multiple owner names because of aliases.  The AA bit

Mockapetris [Page 26]

RFC 1035 Domain Implementation and Specification November 1987

                corresponds to the name which matches the query name, or
                the first owner name in the answer section.

TC TrunCation - specifies that this message was truncated
due to length greater than that permitted on the
transmission channel.

RD Recursion Desired - this bit may be set in a query and
is copied into the response. If RD is set, it directs
the name server to pursue the query recursively.
Recursive query support is optional.

RA Recursion Available - this be is set or cleared in a
response, and denotes whether recursive query support is
available in the name server.

Z Reserved for future use. Must be zero in all queries
and responses.

RCODE Response code - this 4 bit field is set as part of
responses. The values have the following
interpretation:

                0               No error condition

                1               Format error - The name server was
                                unable to interpret the query.

                2               Server failure - The name server was
                                unable to process this query due to a
                                problem with the name server.

                3               Name Error - Meaningful only for
                                responses from an authoritative name
                                server, this code signifies that the
                                domain name referenced in the query does
                                not exist.

                4               Not Implemented - The name server does
                                not support the requested kind of query.

                5               Refused - The name server refuses to
                                perform the specified operation for
                                policy reasons.  For example, a name
                                server may not wish to provide the
                                information to the particular requester,
                                or a name server may not wish to perform
                                a particular operation (e.g., zone

Mockapetris [Page 27]

RFC 1035 Domain Implementation and Specification November 1987

                                transfer) for particular data.

                6-15            Reserved for future use.

QDCOUNT an unsigned 16 bit integer specifying the number of
entries in the question section.

ANCOUNT an unsigned 16 bit integer specifying the number of
resource records in the answer section.

NSCOUNT an unsigned 16 bit integer specifying the number of name
server resource records in the authority records
section.

ARCOUNT an unsigned 16 bit integer specifying the number of
resource records in the additional records section.

4.1.2. Question section format

The question section is used to carry the "question" in most queries,
i.e., the parameters that define what is being asked. The section
contains QDCOUNT (usually 1) entries, each of the following format:

                                    1  1  1  1  1  1
      0  1  2  3  4  5  6  7  8  9  0  1  2  3  4  5
    +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    |                                               |
    /                     QNAME                     /
    /                                               /
    +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    |                     QTYPE                     |
    +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    |                     QCLASS                    |
    +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+

where:

QNAME a domain name represented as a sequence of labels, where
each label consists of a length octet followed by that
number of octets. The domain name terminates with the
zero length octet for the null label of the root. Note
that this field may be an odd number of octets; no
padding is used.

QTYPE a two octet code which specifies the type of the query.
The values for this field include all codes valid for a
TYPE field, together with some more general codes which
can match more than one type of RR.

Mockapetris [Page 28]

RFC 1035 Domain Implementation and Specification November 1987

QCLASS a two octet code that specifies the class of the query.
For example, the QCLASS field is IN for the Internet.

4.1.3. Resource record format

The answer, authority, and additional sections all share the same
format: a variable number of resource records, where the number of
records is specified in the corresponding count field in the header.
Each resource record has the following format:
1 1 1 1 1 1
0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5
+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
| |
/ /
/ NAME /
| |
+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
| TYPE |
+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
| CLASS |
+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
| TTL |
| |
+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
| RDLENGTH |
+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--|
/ RDATA /
/ /
+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+

where:

NAME a domain name to which this resource record pertains.

TYPE two octets containing one of the RR type codes. This
field specifies the meaning of the data in the RDATA
field.

CLASS two octets which specify the class of the data in the
RDATA field.

TTL a 32 bit unsigned integer that specifies the time
interval (in seconds) that the resource record may be
cached before it should be discarded. Zero values are
interpreted to mean that the RR can only be used for the
transaction in progress, and should not be cached.

Mockapetris [Page 29]

RFC 1035 Domain Implementation and Specification November 1987

RDLENGTH an unsigned 16 bit integer that specifies the length in
octets of the RDATA field.

RDATA a variable length string of octets that describes the
resource. The format of this information varies
according to the TYPE and CLASS of the resource record.
For example, the if the TYPE is A and the CLASS is IN,
the RDATA field is a 4 octet ARPA Internet address.

4.1.4. Message compression

In order to reduce the size of messages, the domain system utilizes a
compression scheme which eliminates the repetition of domain names in a
message. In this scheme, an entire domain name or a list of labels at
the end of a domain name is replaced with a pointer to a prior occurance
of the same name.

The pointer takes the form of a two octet sequence:

    +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    | 1  1|                OFFSET                   |
    +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+

The first two bits are ones. This allows a pointer to be distinguished
from a label, since the label must begin with two zero bits because
labels are restricted to 63 octets or less. (The 10 and 01 combinations
are reserved for future use.) The OFFSET field specifies an offset from
the start of the message (i.e., the first octet of the ID field in the
domain header). A zero offset specifies the first byte of the ID field,
etc.

The compression scheme allows a domain name in a message to be
represented as either:

- a sequence of labels ending in a zero octet

- a pointer

- a sequence of labels ending with a pointer

Pointers can only be used for occurances of a domain name where the
format is not class specific. If this were not the case, a name server
or resolver would be required to know the format of all RRs it handled.
As yet, there are no such cases, but they may occur in future RDATA
formats.

If a domain name is contained in a part of the message subject to a
length field (such as the RDATA section of an RR), and compression is

Mockapetris [Page 30]

RFC 1035 Domain Implementation and Specification November 1987

used, the length of the compressed name is used in the length
calculation, rather than the length of the expanded name.

Programs are free to avoid using pointers in messages they generate,
although this will reduce datagram capacity, and may cause truncation.
However all programs are required to understand arriving messages that
contain pointers.

For example, a datagram might need to use the domain names F.ISI.ARPA,
FOO.F.ISI.ARPA, ARPA, and the root. Ignoring the other fields of the
message, these domain names might be represented as:

       +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    20 |           1           |           F           |
       +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    22 |           3           |           I           |
       +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    24 |           S           |           I           |
       +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    26 |           4           |           A           |
       +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    28 |           R           |           P           |
       +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    30 |           A           |           0           |
       +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+

       +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    40 |           3           |           F           |
       +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    42 |           O           |           O           |
       +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    44 | 1  1|                20                       |
       +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+

       +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    64 | 1  1|                26                       |
       +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+

       +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
    92 |           0           |                       |
       +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+

The domain name for F.ISI.ARPA is shown at offset 20. The domain name
FOO.F.ISI.ARPA is shown at offset 40; this definition uses a pointer to
concatenate a label for FOO to the previously defined F.ISI.ARPA. The
domain name ARPA is defined at offset 64 using a pointer to the ARPA
component of the name F.ISI.ARPA at 20; note that this pointer relies on
ARPA being the last label in the string at 20. The root domain name is

Mockapetris [Page 31]

RFC 1035 Domain Implementation and Specification November 1987

defined by a single octet of zeros at 92; the root domain name has no
labels.
